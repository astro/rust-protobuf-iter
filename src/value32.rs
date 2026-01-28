/// Encapsulated for From
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Value32<'a> {
    pub data: &'a [u8],
}

impl<'a> From<Value32<'a>> for u32 {
    fn from(value32: Value32<'a>) -> u32 {
        u32::from_le_bytes(value32.data.try_into().unwrap())
    }
}

impl<'a> From<Value32<'a>> for i32 {
    fn from(value32: Value32<'a>) -> i32 {
        i32::from_le_bytes(value32.data.try_into().unwrap())
    }
}

impl<'a> From<Value32<'a>> for u64 {
    fn from(value32: Value32<'a>) -> u64 {
        u32::from_le_bytes(value32.data.try_into().unwrap()) as u64
    }
}

impl<'a> From<Value32<'a>> for i64 {
    fn from(value32: Value32<'a>) -> i64 {
        i32::from_le_bytes(value32.data.try_into().unwrap()) as i64
    }
}
