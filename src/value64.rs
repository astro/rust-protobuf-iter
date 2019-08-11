use byteorder::{ByteOrder, LittleEndian};


/// Encapsulated for From
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Value64<'a> {
    pub data: &'a [u8]
}

impl<'a> From<Value64<'a>> for u32 {
    fn from(value64: Value64<'a>) -> u32 {
        LittleEndian::read_u64(&value64.data) as u32
    }
}

impl<'a> From<Value64<'a>> for i32 {
    fn from(value64: Value64<'a>) -> i32 {
        LittleEndian::read_i64(&value64.data) as i32
    }
}

impl<'a> From<Value64<'a>> for u64 {
    fn from(value64: Value64<'a>) -> u64 {
        LittleEndian::read_u64(&value64.data)
    }
}

impl<'a> From<Value64<'a>> for i64 {
    fn from(value64: Value64<'a>) -> i64 {
        LittleEndian::read_i64(&value64.data)
    }
}
