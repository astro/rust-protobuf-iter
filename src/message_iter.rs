use std::convert::From;
use std::marker::PhantomData;
use message::*;
use parse_message::*;


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

    pub fn tag<T: From<ParseValue<&'a [u8]>>>(self, tag: u64) -> ByTag<'a, T> {
        ByTag {
            tag: tag,
            inner: self,
            items: PhantomData
        }
    }
}

/// Recursive for LengthDelimited
impl<'a> From<ParseValue<&'a [u8]>> for MessageIter<'a> {
    fn from(value: ParseValue<&'a [u8]>) -> MessageIter<'a> {
        match value {
            ParseValue::LengthDelimited(data) =>
                MessageIter::new(data.as_ref()),
            _ =>
                panic!("Expected buffer to parse")
        }
    }
}

impl<'a> Iterator for MessageIter<'a> {
    type Item = Message<&'a [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        match parse_message(self.data) {
            ParseResult::Result((tag, value, rest)) => {
                self.data = rest;
                Some(Message {
                    tag: tag,
                    value: value
                })
            },
            _ => None
        }
    }
}

/// Returned by MessageIter.tag()
#[derive(Clone)]
pub struct ByTag<'a, T: 'a + From<ParseValue<&'a [u8]>>> {
    tag: u64,
    inner: MessageIter<'a>,
    items: PhantomData<&'a T>
}

impl<'a, T: 'a + From<ParseValue<&'a [u8]>>> Iterator for ByTag<'a, T> {
    // type Item = ParseValue<&'a [u8]>;
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .and_then(|msg| {
                if msg.tag == self.tag {
                    Some(From::from(msg.value))
                } else {
                    None
                }
            })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use message::*;
    use parse_message::*;

    #[test]
    fn nested_iter() {
        let data = [0x1a, 0x03, 0x08, 0x96, 0x01];
        let mut outer = MessageIter::new(&data);
        assert_eq!(outer.next(), Some(Message {
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
