use std::convert::From;
use std::marker::PhantomData;
use field::*;
use parse::*;


#[derive(Clone)]
pub struct MessageIter<'a> {
    data: &'a [u8]
}

impl<'a> MessageIter<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        MessageIter {
            data: data
        }
    }

    pub fn tag<T: From<ParseValue<'a>>>(self, tag: u32) -> ByTag<'a, T> {
        ByTag {
            tag: tag,
            inner: self,
            items: PhantomData
        }
    }
}

/// Recursive for LengthDelimited
impl<'a> From<ParseValue<'a>> for MessageIter<'a> {
    fn from(value: ParseValue<'a>) -> MessageIter<'a> {
        match value {
            ParseValue::LengthDelimited(data) =>
                MessageIter::new(data.as_ref()),
            _ =>
                panic!("Expected buffer to parse")
        }
    }
}

impl<'a> Iterator for MessageIter<'a> {
    type Item = Field<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        parse_field(self.data)
            .map(|(field, rest)| {
                self.data = rest;
                field
            })
            .ok()
    }
}

/// Returned by MessageIter.tag()
#[derive(Clone)]
pub struct ByTag<'a, T: 'a + From<ParseValue<'a>>> {
    tag: u32,
    inner: MessageIter<'a>,
    items: PhantomData<&'a T>
}

impl<'a, T: 'a + From<ParseValue<'a>>> Iterator for ByTag<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .and_then(|msg| {
                if msg.tag == self.tag {
                    Some(From::from(msg.value))
                } else {
                    self.next()
                }
            })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_iter() {
        let data = [0x1a, 0x03, 0x08, 0x96, 0x01];
        let mut outer = MessageIter::new(&data);
        assert_eq!(outer.next(), Some(Field {
            tag: 3,
            value: ParseValue::LengthDelimited(&[0x08, 0x96, 0x01][..])
        }));
        assert_eq!(outer.next(), None);

        let data = [0x08, 0x96, 0x01];
        let mut inner = MessageIter::new(&data);
        let m = inner.next().unwrap();
        assert_eq!(1, m.tag);
        assert_eq!(150u32, From::from(m.value));
        assert_eq!(inner.next(), None);
    }

    #[test]
    fn by_tag() {
        let data = [0x08, 0x96, 0x01];
        let iter = MessageIter::new(&data);
        let r: Vec<u32> = iter.tag(1)
            .collect();
        assert_eq!(vec![150u32], r);
    }

    #[test]
    fn nested_by_tag() {
        let data = [0x1a, 0x03, 0x08, 0x96, 0x01];
        let iter = MessageIter::new(&data);
        let c = iter.tag::<MessageIter>(3).nth(0).unwrap();
        let a = c.tag::<u32>(1).nth(0);
        assert_eq!(Some(150u32), a);
    }
}
