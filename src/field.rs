use crate::parse::ParseValue;

#[derive(Debug, Eq, PartialEq)]
pub struct Field<'a> {
    pub tag: u32,
    pub value: ParseValue<'a>,
}

impl<'a> From<Field<'a>> for ParseValue<'a> {
    fn from(m: Field<'a>) -> Self {
        m.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_field;
    use crate::varint::Varint;

    #[test]
    fn test_parse_value_from_field() {
        let data = [0x08, 0x96, 0x01];
        let (field, _rest) = parse_field(&data).unwrap();
        assert_eq!(
            ParseValue::from(field),
            ParseValue::Varint(Varint { value: 150 })
        );
    }
}
