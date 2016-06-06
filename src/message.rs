use parse_message::ParseValue;


#[derive(Debug, Eq, PartialEq)]
pub struct Message<C> {
    pub tag: u64,
    pub value: ParseValue<C>
}

impl<C> From<Message<C>> for ParseValue<C> {
    fn from(m: Message<C>) -> Self {
        m.value
    }
}
