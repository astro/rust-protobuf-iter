use std::marker::PhantomData;

use crate::parse::*;
use crate::value32::*;
use crate::value64::*;
use crate::varint::*;

pub trait Packed<'a> {
    type Item;

    fn parse(data: &'a [u8]) -> ParseResult<(Self::Item, &'a [u8])>;
}

#[derive(Clone, Copy)]
pub struct PackedVarint;

impl<'a> Packed<'a> for PackedVarint {
    type Item = Varint;

    fn parse(data: &'a [u8]) -> ParseResult<(Varint, &'a [u8])> {
        parse_varint(data)
    }
}

#[derive(Clone, Copy)]
pub struct PackedValue32;

impl<'a> Packed<'a> for PackedValue32 {
    type Item = Value32<'a>;

    fn parse(data: &'a [u8]) -> ParseResult<(Value32<'a>, &'a [u8])> {
        parse_value32(data)
    }
}

#[derive(Clone, Copy)]
pub struct PackedValue64;

impl<'a> Packed<'a> for PackedValue64 {
    type Item = Value64<'a>;

    fn parse(data: &'a [u8]) -> ParseResult<(Value64<'a>, &'a [u8])> {
        parse_value64(data)
    }
}
                                                 

#[derive(Clone)]
pub struct PackedIter<'a, P, T> {
    data: &'a [u8],
    packed: PhantomData<P>,
    item: PhantomData<T>
}

impl<'a, P, T> PackedIter<'a, P, T> {
    pub fn new(data: &'a [u8]) -> Self {
        PackedIter {
            data: data,
            packed: PhantomData,
            item: PhantomData
        }
    }
}

impl<'a, P, T> From<ParseValue<'a>> for PackedIter<'a, P, T> {
    fn from(parse_value: ParseValue<'a>) -> Self {
        Self::new(parse_value.get_data())
    }
}

/// Type parameter P: Encoding
/// Type parameter T: Coercion target
impl<'a, P: Packed<'a>, T: From<<P as Packed<'a>>::Item>> Iterator for PackedIter<'a, P, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() < 1 {
            return None
        }

        P::parse(self.data)
            .ok()
            .map(|(value, rest)| {
                self.data = rest;
                From::from(value)
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.data.len()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const VARINTS_ENCODED: &'static [u8] = &[0x03, 0x8E, 0x02, 0x9E, 0xA7, 0x05];
    
    #[test]
    fn packed_varints() {
        let iter: PackedIter<'static, PackedVarint, u32> = PackedIter::new(VARINTS_ENCODED);
        assert_eq!(vec![3, 270, 86942], iter.collect::<Vec<u32>>());
    }
    
    const VALUE32S_ENCODED: &'static [u8] = &[
        1, 0, 0, 0,
        2, 0, 0, 0,
        3, 0, 0, 0
    ];

    #[test]
    fn packed_value32s() {
        let iter: PackedIter<'static, PackedValue32, u32> = PackedIter::new(VALUE32S_ENCODED);
        assert_eq!(vec![1, 2, 3], iter.collect::<Vec<u32>>());
    }
    
    const VALUE64S_ENCODED: &'static [u8] = &[
        1, 0, 0, 0, 0, 0, 0, 0,
        2, 0, 0, 0, 0, 0, 0, 0,
        3, 0, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn packed_value64s() {
        let iter: PackedIter<'static, PackedValue64, u32> = PackedIter::new(VALUE64S_ENCODED);
        assert_eq!(vec![1, 2, 3], iter.collect::<Vec<u32>>());
    }
}
