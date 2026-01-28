/// Encapsulated for From
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Value64<'a> {
    pub data: &'a [u8]
}

impl<'a> From<Value64<'a>> for u32 {
    fn from(value64: Value64<'a>) -> u32 {
	u64::from_le_bytes(value64.data.try_into().unwrap()) as u32
    }
}

impl<'a> From<Value64<'a>> for i32 {
    fn from(value64: Value64<'a>) -> i32 {
	i64::from_le_bytes(value64.data.try_into().unwrap()) as i32
    }
}

impl<'a> From<Value64<'a>> for u64 {
    fn from(value64: Value64<'a>) -> u64 {
	u64::from_le_bytes(value64.data.try_into().unwrap())
    }
}

impl<'a> From<Value64<'a>> for i64 {
    fn from(value64: Value64<'a>) -> i64 {
	i64::from_le_bytes(value64.data.try_into().unwrap())
    }
}
