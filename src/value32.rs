use byteorder::{ByteOrder, LittleEndian};


/// Encapsulated for From
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Value32<'a> {
    pub data: &'a [u8]
}

impl<'a> From<Value32<'a>> for u32 {
    fn from(value32: Value32<'a>) -> u32 {
        LittleEndian::read_u32(&value32.data)
    }
}

impl<'a> From<Value32<'a>> for i32 {
    fn from(value32: Value32<'a>) -> i32 {
        LittleEndian::read_i32(&value32.data)
    }
}

impl<'a> From<Value32<'a>> for u64 {
    fn from(value32: Value32<'a>) -> u64 {
        LittleEndian::read_u32(&value32.data) as u64
    }
}

impl<'a> From<Value32<'a>> for i64 {
    fn from(value32: Value32<'a>) -> i64 {
        LittleEndian::read_i32(&value32.data) as i64
    }
}
