use parse::ParseValue;


#[derive(Debug, Eq, PartialEq)]
pub struct Field<'a> {
    pub tag: u32,
    pub value: ParseValue<'a>
}

impl<'a> From<Field<'a>> for ParseValue<'a> {
    fn from(m: Field<'a>) -> Self {
        m.value
    }
}
