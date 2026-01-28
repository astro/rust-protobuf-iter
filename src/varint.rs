use crate::zigzag::ZigZag;

/// Encapsulated for From
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Varint {
    pub value: u64
}

impl From<Varint> for u32 {
    fn from(varint: Varint) -> u32 {
        varint.value as u32
    }
}

impl From<Varint> for i32 {
    fn from(varint: Varint) -> i32 {
        varint.value.zigzag() as i32
    }
}

impl From<Varint> for u64 {
    fn from(varint: Varint) -> u64 {
        varint.value
    }
}

impl From<Varint> for i64 {
    fn from(varint: Varint) -> i64 {
        varint.value.zigzag()
    }
}
