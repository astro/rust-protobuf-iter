use std::convert::From;
use byteorder::{ByteOrder, LittleEndian};

use packed::*;
use zigzag::ZigZag;


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ParseValue<C> {
    Value32(C),
    Value64(C),
    Varint(u64),
    LengthDelimited(C),
}

impl<'a> From<ParseValue<&'a [u8]>> for &'a [u8] {
    fn from(input: ParseValue<&'a [u8]>) -> Self {
        input.get_data()
    }
}

impl<C: AsRef<[u8]>> From<ParseValue<C>> for u32 {
    fn from(value: ParseValue<C>) -> u32 {
        match value {
            ParseValue::Value32(data) =>
                LittleEndian::read_u32(data.as_ref()),
            ParseValue::Value64(data) =>
                LittleEndian::read_u32(&data.as_ref()[4..]),
            ParseValue::Varint(i) => i as u32,
            _ => 0
        }
    }
}

impl<C: AsRef<[u8]>> From<ParseValue<C>> for i32 {
    fn from(value: ParseValue<C>) -> i32 {
        let r: u32 = From::from(value);
        r.zigzag()
    }
}

impl<C: AsRef<[u8]>> From<ParseValue<C>> for u64 {
    fn from(value: ParseValue<C>) -> u64 {
        match value {
            ParseValue::Value32(data) =>
                LittleEndian::read_u32(data.as_ref()) as u64,
            ParseValue::Value64(data) =>
                LittleEndian::read_u64(data.as_ref()),
            ParseValue::Varint(i) => i,
            _ => 0
        }
    }
}

impl<C: AsRef<[u8]>> From<ParseValue<C>> for i64 {
    fn from(value: ParseValue<C>) -> i64 {
        let r: u64 = From::from(value);
        r.zigzag()
    }
}

/// Getters for packed values
impl<'a> ParseValue<&'a [u8]> {
    pub fn get_data(self) -> &'a [u8] {
        match self {
            ParseValue::LengthDelimited(ref data) =>
                data,
            _ =>
                panic!("Expected length-delimited data")
        }
    }

    pub fn packed_varints<T>(self) -> PackedIter<'a, PackedVarint, T> {
        PackedIter::new(self.get_data())
    }
}


// TODO: Result<A, ParseError>
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParseResult<A> {
    Result(A),
    NotEnoughData,
    Error(String)
}

fn parse_value32_value<'a>(data: &'a [u8]) -> ParseResult<(ParseValue<&'a [u8]>, &'a [u8])> {
    if data.len() >= 8 {
        ParseResult::Result((ParseValue::Value64(&data[0..8]), &data[8..]))
    } else {
        ParseResult::NotEnoughData
    }
}

fn parse_value64_value<'a>(data: &'a [u8]) -> ParseResult<(ParseValue<&'a [u8]>, &'a [u8])> {
    if data.len() >= 8 {
        ParseResult::Result((ParseValue::Value64(&data[0..8]), &data[8..]))
    } else {
        ParseResult::NotEnoughData
    }
}

pub fn parse_varint_value<'a>(data: &'a [u8]) -> ParseResult<(ParseValue<&'a [u8]>, &'a [u8])> {
    let mut value = 0;
    let mut i = 0;
    while i < data.len() && data[i] & 0x80 != 0 {
        value |= ((data[i] & 0x7f) as u64) << (7 * i);
        i += 1;
    }
    if i < data.len() {
        value |= (data[i] as u64) << (7 * i);
        ParseResult::Result((ParseValue::Varint(value), &data[(i + 1)..]))
    } else {
        ParseResult::NotEnoughData
    }
}

fn parse_length_delimited_value<'a>(data: &'a [u8]) -> ParseResult<(ParseValue<&'a [u8]>, &'a [u8])> {
    match parse_varint_value(data) {
        ParseResult::Result((ParseValue::Varint(len), data)) => {
            let len = len as usize;
            ParseResult::Result((ParseValue::LengthDelimited(&data[0..len]), &data[len..]))
        },
        condition => condition
    }
}

fn parse_deprecated_value<'a>(_: &'a [u8]) -> ParseResult<(ParseValue<&'a [u8]>, &'a [u8])> {
    ParseResult::Error("Deprecated groups field".to_owned())
}
                                                
fn parse_invalid_type<'a>(_: &'a [u8]) -> ParseResult<(ParseValue<&'a [u8]>, &'a [u8])> {
    ParseResult::Error("Invalid type".to_owned())
}

// TODO: could return a Message
// TODO: Message.tag: u32
pub fn parse_message<'a>(data: &'a [u8]) -> ParseResult<(u64, ParseValue<&'a [u8]>, &'a [u8])> {
    let (key, data) = match parse_varint_value(data) {
        ParseResult::Result((ParseValue::Varint(key), data)) =>
            (key, data),
        ParseResult::Result(_) =>
            return ParseResult::Error("Not varint".to_owned()),
        ParseResult::NotEnoughData =>
            return ParseResult::NotEnoughData,
        ParseResult::Error(s) =>
            return ParseResult::Error(s)
    };
    let msg_tag = key >> 3;
    let msg_type = key & 7;

    let msg_actions = [
        parse_varint_value,
        parse_value64_value,
        parse_length_delimited_value,
        parse_deprecated_value,
        parse_deprecated_value,
        parse_value32_value,
        parse_invalid_type,
        parse_invalid_type,
    ];
    assert_eq!(msg_actions.len(), 8);
    let msg_action = msg_actions[msg_type as usize];
    match msg_action(data) {
        ParseResult::Result((value, data)) =>
            ParseResult::Result((msg_tag, value, data)),
        ParseResult::NotEnoughData =>
            ParseResult::NotEnoughData,
        ParseResult::Error(s) => {
            println!("Cannot parse tag {} type {}: {}", msg_tag, msg_type, s);
            ParseResult::Error(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings() {
        let data = [0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
        let r = parse_message(&data);
        assert_eq!(r, ParseResult::Result((2, ParseValue::LengthDelimited(b"testing".as_ref()), &[][..])));
    }

    #[test]
    fn varint() {
        let data = [0x08, 0x96, 0x01];
        let r = parse_message(&data);
        assert_eq!(r, ParseResult::Result((1, ParseValue::Varint(150), &[][..])));
    }

    #[test]
    fn varint_value() {
        let data = [0b10101100, 0b00000010];
        let r = super::parse_varint_value(&data);
        assert_eq!(r, ParseResult::Result((ParseValue::Varint(300), &[][..])));
    }

    #[test]
    fn typed() {
        let value32 = ParseValue::Value32(vec![0x96, 0, 0, 0]);
        assert_eq!(150u32, From::from(value32.clone()));
        assert_eq!(75i32, From::from(value32.clone()));
        assert_eq!(150u64, From::from(value32.clone()));
        assert_eq!(75i64, From::from(value32.clone()));

        let value64 = ParseValue::Value64(vec![0x96, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(150u64, From::from(value64.clone()));
        assert_eq!(75i64, From::from(value64.clone()));
        assert_eq!(150u64, From::from(value64.clone()));
        assert_eq!(75i64, From::from(value64.clone()));

        let varint: ParseValue<Vec<u8>> = ParseValue::Varint(150);
        assert_eq!(150u64, From::from(varint.clone()));
        assert_eq!(75i64, From::from(varint.clone()));
        assert_eq!(150u64, From::from(varint.clone()));
        assert_eq!(75i64, From::from(varint.clone()));
    }

    #[test]
    fn typed_buffer() {
        let delimited: ParseValue<&'static [u8]> = ParseValue::LengthDelimited(b"testing");
        let value: &'static [u8] = From::from(delimited);
        assert_eq!(b"testing", value);
    }

    #[test]
    fn packed_varints() {
        let data = [0x22, 0x06, 0x03, 0x8E, 0x02, 0x9E, 0xA7, 0x05];
        match parse_message(&data) {
            ParseResult::Result((tag, value, rest)) => {
                assert_eq!(tag, 4);
                assert_eq!(vec![3, 270, 86942], value.packed_varints::<u32>().collect::<Vec<u32>>());
                assert_eq!(rest.len(), 0);
            },
            _ => {
                assert!(false);
            }
        }
    }
}
