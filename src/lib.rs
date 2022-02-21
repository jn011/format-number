struct Number {
    value: i64,
}

impl Number {
    pub fn new(value: &str) -> Number {
        Number { value: 2 }
    }
}

enum NumberType {
    Decimal,
    Hexadecimal,
    Binary,
}

struct NumberFactory;
impl NumberFactory {
    pub fn new_number(number_type: &NumberType) {
        match number_type {
            NumberType::Decimal => todo!(),
            NumberType::Hexadecimal => todo!(),
            NumberType::Binary => todo!(),
            _ => todo!(),
        }
    }
}
