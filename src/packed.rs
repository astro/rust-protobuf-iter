use std::marker::PhantomData;

use parse::*;
use varint::*;

pub trait Packed {
    type Item;

    fn parse<'a>(&'a [u8]) -> ParseResult<(Self::Item, &'a [u8])>;
}

pub struct PackedVarint;

impl Packed for PackedVarint {
    type Item = Varint;

    fn parse<'a>(data: &'a [u8]) -> ParseResult<(Varint, &'a [u8])> {
        parse_varint(data)
    }
}

// TODO: PackedValue32, PackedValue64


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

/// Type parameter P: Encoding
/// Type parameter T: Coercion target
impl<'a, P: Packed, T: From<<P as Packed>::Item>> Iterator
for PackedIter<'a, P, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() < 1 {
            return None
        }

        match P::parse(self.data) {
            Ok((value, rest)) => {
                self.data = rest;
                Some(From::from(value))
            },
            _ => {
                None
            }
        }
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
}
