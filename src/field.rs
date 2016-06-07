use parse::ParseValue;


#[derive(Debug, Eq, PartialEq)]
pub struct Field<C> {
    pub tag: u32,
    pub value: ParseValue<C>
}

impl<C> From<Field<C>> for ParseValue<C> {
    fn from(m: Field<C>) -> Self {
        m.value
    }
}
