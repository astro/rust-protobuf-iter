use std::convert::From;
use std::ops::Deref;

use field::*;
use packed::*;
use value32::*;
use value64::*;
use varint::*;

    
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ParseValue<'a> {
    Value32(Value32<'a>),
    Value64(Value64<'a>),
    Varint(Varint),
    LengthDelimited(&'a [u8]),
}

impl<'a> Deref for ParseValue<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            &ParseValue::LengthDelimited(ref data) => data,
            _ => panic!("Expected length-delimited data")
        }
    }
}

impl<'a> From<ParseValue<'a>> for &'a [u8] {
    fn from(input: ParseValue<'a>) -> Self {
        input.get_data()
    }
}

impl<'a> From<ParseValue<'a>> for u32 {
    fn from(value: ParseValue<'a>) -> u32 {
        match value {
            ParseValue::Value32(value32) =>
                From::from(value32),
            ParseValue::Value64(value64) =>
                From::from(value64),
            ParseValue::Varint(varint) =>
                From::from(varint),
            _ =>
                0
        }
    }
}

impl<'a> From<ParseValue<'a>> for i32 {
    fn from(value: ParseValue<'a>) -> i32 {
        match value {
            ParseValue::Value32(value32) =>
                From::from(value32),
            ParseValue::Value64(value64) =>
                From::from(value64),
            ParseValue::Varint(varint) =>
                From::from(varint),
            _ =>
                0
        }
    }
}

impl<'a> From<ParseValue<'a>> for u64 {
    fn from(value: ParseValue<'a>) -> u64 {
        match value {
            ParseValue::Value32(value32) =>
                From::from(value32),
            ParseValue::Value64(value64) =>
                From::from(value64),
            ParseValue::Varint(varint) =>
                From::from(varint),
            _ =>
                0
        }
    }
}

impl<'a> From<ParseValue<'a>> for i64 {
    fn from(value: ParseValue<'a>) -> i64 {
        match value {
            ParseValue::Value32(value32) =>
                From::from(value32),
            ParseValue::Value64(value64) =>
                From::from(value64),
            ParseValue::Varint(varint) =>
                From::from(varint),
            _ =>
                0
        }
    }
}

/// Getters for packed values
impl<'a> ParseValue<'a> {
    pub fn get_data(self) -> &'a [u8] {
        match self {
            ParseValue::LengthDelimited(ref data) =>
                data,
            _ =>
                // TODO: Option?
                panic!("Expected length-delimited data")
        }
    }

    pub fn packed_varints<T>(self) -> PackedIter<'a, PackedVarint, T> {
        PackedIter::new(self.get_data())
    }

    pub fn packed_value32s<T>(self) -> PackedIter<'a, PackedValue32, T> {
        PackedIter::new(self.get_data())
    }

    pub fn packed_value64s<T>(self) -> PackedIter<'a, PackedValue64, T> {
        PackedIter::new(self.get_data())
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
    NotEnoughData,
    DeprecatedType,
    InvalidType,
    Unexpected,
}
            
pub type ParseResult<A> = Result<A, ParseError>;

pub fn parse_value32<'a>(data: &'a [u8]) -> ParseResult<(Value32<'a>, &'a [u8])> {
    if data.len() >= 4 {
        Ok((Value32 { data: &data[0..4] }, &data[4..]))
    } else {
        Err(ParseError::NotEnoughData)
    }
}

fn parse_value32_value<'a>(data: &'a [u8]) -> ParseResult<(ParseValue<'a>, &'a [u8])> {
    parse_value32(data)
        .map(|(value32, rest)| (ParseValue::Value32(value32), rest))
}

pub fn parse_value64<'a>(data: &'a [u8]) -> ParseResult<(Value64<'a>, &'a [u8])> {
    if data.len() >= 8 {
        Ok((Value64 { data: &data[0..8] }, &data[8..]))
    } else {
        Err(ParseError::NotEnoughData)
    }
}

fn parse_value64_value<'a>(data: &'a [u8]) -> ParseResult<(ParseValue<'a>, &'a [u8])> {
    parse_value64(data)
        .map(|(value64, rest)| (ParseValue::Value64(value64), rest))
}
    
/// Used by packed::PackedVarint to avoid the detour over distinguishing between ParseValue members
pub fn parse_varint<'a>(data: &'a [u8]) -> ParseResult<(Varint, &'a [u8])> {
    let mut value = 0;
    let mut i = 0;
    while i < data.len() && data[i] & 0x80 != 0 {
        value |= ((data[i] & 0x7f) as u64) << (7 * i);
        i += 1;
    }
    if i < data.len() {
        value |= (data[i] as u64) << (7 * i);
        Ok((Varint { value: value }, &data[(i + 1)..]))
    } else {
        Err(ParseError::NotEnoughData)
    }
}

fn parse_varint_value<'a>(data: &'a [u8]) -> ParseResult<(ParseValue<'a>, &'a [u8])> {
    parse_varint(data)
        .map(|(varint, rest)| (ParseValue::Varint(varint), rest))
}

fn parse_length_delimited_value<'a>(data: &'a [u8]) -> ParseResult<(ParseValue<'a>, &'a [u8])> {
    match parse_varint_value(data) {
        Ok((ParseValue::Varint(len), data)) => {
            let len: u64 = From::from(len);
            let len = len as usize;
            Ok((ParseValue::LengthDelimited(&data[0..len]), &data[len..]))
        },
        condition => condition
    }
}

fn parse_deprecated_value<'a>(_: &'a [u8]) -> ParseResult<(ParseValue<'a>, &'a [u8])> {
    Err(ParseError::DeprecatedType)
}
                                                
fn parse_invalid_type<'a>(_: &'a [u8]) -> ParseResult<(ParseValue<'a>, &'a [u8])> {
    Err(ParseError::InvalidType)
}

pub fn parse_field<'a>(data: &'a [u8]) -> ParseResult<(Field<'a>, &'a [u8])> {
    let (key, data) = match parse_varint_value(data) {
        Ok((ParseValue::Varint(key), data)) => {
            let key: u64 = From::from(key);
            (key, data)
        },
        Ok(_) =>
            return Err(ParseError::Unexpected),
        Err(e) =>
            return Err(e)
    };

    let msg_tag = (key >> 3) as u32;
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
        Ok((value, data)) =>
            Ok((Field {
                tag: msg_tag,
                value: value
            }, data)),
        Err(e) =>
            Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use field::*;
    use value32::*;
    use value64::*;
    use varint::*;

    #[test]
    fn strings() {
        let data = [0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
        let r = parse_field(&data).unwrap();
        assert_eq!(r, (Field {
            tag: 2,
            value: ParseValue::LengthDelimited(b"testing".as_ref())
        }, &[][..]));
    }

    #[test]
    fn varint() {
        let data = [0x08, 0x96, 0x01];
        let r = parse_field(&data).unwrap();
        assert_eq!(r, (Field {
            tag: 1,
            value: ParseValue::Varint(Varint { value: 150 })
        }, &[][..]));
    }

    #[test]
    fn varint_value() {
        let data = [0b10101100, 0b00000010];
        let r = super::parse_varint_value(&data).unwrap();
        assert_eq!(r, (ParseValue::Varint(Varint { value: 300 }), &[][..]));
    }

    #[test]
    fn typed() {
        let data32 = &[0x96, 0, 0, 0];
        let value32 = ParseValue::Value32(Value32 { data: data32 });
        assert_eq!(150u32, From::from(value32.clone()));
        assert_eq!(150i32, From::from(value32.clone()));
        assert_eq!(150u64, From::from(value32.clone()));
        assert_eq!(150i64, From::from(value32.clone()));

        let data64 = &[0x96, 0, 0, 0, 0, 0, 0, 0];
        let value64 = ParseValue::Value64(Value64 { data: data64 });
        assert_eq!(150u64, From::from(value64.clone()));
        assert_eq!(150i64, From::from(value64.clone()));
        assert_eq!(150u64, From::from(value64.clone()));
        assert_eq!(150i64, From::from(value64.clone()));

        let varint = ParseValue::Varint(Varint { value: 150 });
        assert_eq!(150u64, From::from(varint.clone()));
        assert_eq!(75i64, From::from(varint.clone()));
        assert_eq!(150u64, From::from(varint.clone()));
        assert_eq!(75i64, From::from(varint.clone()));
    }

    #[test]
    fn typed_buffer() {
        let delimited: ParseValue<'static> = ParseValue::LengthDelimited(b"testing");
        let value: &'static [u8] = From::from(delimited);
        assert_eq!(b"testing", value);
    }

    #[test]
    fn packed_varints() {
        let data = [0x22, 0x06, 0x03, 0x8E, 0x02, 0x9E, 0xA7, 0x05];
        match parse_field(&data) {
            Ok((field, rest)) => {
                assert_eq!(field.tag, 4);
                assert_eq!(vec![3, 270, 86942], field.value.packed_varints::<u32>().collect::<Vec<u32>>());
                assert_eq!(rest.len(), 0);
            },
            _ => {
                assert!(false);
            }
        }
    }
}
