use clap::{ArgEnum, Parser};
use core::fmt;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(ArgEnum, Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberType {
    Integer,
    Hexadecimal,
    Binary,
}

impl NumberType {
    pub fn iter() -> std::slice::Iter<'static, NumberType> {
        static NUMBERTYPES: [NumberType; 3] = [
            NumberType::Integer,
            NumberType::Hexadecimal,
            NumberType::Binary,
        ];
        NUMBERTYPES.iter()
    }
}

impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberType::Integer => write!(f, "Integer"),
            NumberType::Hexadecimal => write!(f, "Hexadecimal"),
            NumberType::Binary => write!(f, "Binary"),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CommandOptions {
    #[clap(short, long, arg_enum, default_value_t=NumberType::Integer)]
    pub number_type: NumberType,
    pub number: String,
}

impl CommandOptions {
    pub fn new(number_type: NumberType, input: &str) -> Self {
        Self {
            number_type,
            number: input.to_string(),
        }
    }
}

pub struct CommandContext {
    command_options: CommandOptions,
}

impl CommandContext {
    pub fn new(command_options: CommandOptions) -> Self {
        Self { command_options }
    }

    pub fn format_all_number_types(
        &self,
    ) -> anyhow::Result<Vec<(NumberType, String)>, NumberFormatterError> {
        let mut vec = Vec::<(NumberType, String)>::new();

        let formatter =
            NumberFormatterFactory::new_number_formatter(&self.command_options.number_type);

        let num = formatter.read(&self.command_options.number)?;

        for number_type in NumberType::iter() {
            let formatter = NumberFormatterFactory::new_number_formatter(number_type);
            let output = formatter.format(num)?;
            vec.push((*number_type, output));
        }

        Ok(vec)
    }
}

#[derive(Error, Debug)]
pub enum NumberFormatterError {
    #[error("Unknown error occurred")]
    Unknown,
    #[error("No value was entered")]
    Empty,
    #[error("Number too large")]
    TooLargeError,
    #[error("Number too small")]
    TooSmallError,
    #[error("Number contains an invalid digit")]
    InvalidDigit,
}

impl From<ParseIntError> for NumberFormatterError {
    fn from(e: ParseIntError) -> Self {
        match &e.kind() {
            std::num::IntErrorKind::Empty => NumberFormatterError::Empty,
            std::num::IntErrorKind::InvalidDigit => NumberFormatterError::InvalidDigit,
            std::num::IntErrorKind::PosOverflow => NumberFormatterError::TooLargeError,
            std::num::IntErrorKind::NegOverflow => NumberFormatterError::TooSmallError,
            _ => NumberFormatterError::Unknown,
        }
    }
}

trait NumberFormatter {
    fn read(&self, num: &str) -> anyhow::Result<i128, NumberFormatterError>;
    fn format(&self, num: i128) -> anyhow::Result<String, NumberFormatterError>;
}

struct IntegerNumberFormatter;
impl NumberFormatter for IntegerNumberFormatter {
    fn read(&self, integer: &str) -> anyhow::Result<i128, NumberFormatterError> {
        integer.parse::<i128>().map_err(|op| op.into())
    }

    fn format(&self, num: i128) -> anyhow::Result<String, NumberFormatterError> {
        Ok(num.to_string())
    }
}

struct HexadecimalNumberFormatter;
impl NumberFormatter for HexadecimalNumberFormatter {
    fn read(&self, hexadecimal: &str) -> anyhow::Result<i128, NumberFormatterError> {
        let without_prefix = hexadecimal.trim_start_matches("0x");
        i128::from_str_radix(without_prefix, 16).map_err(|op| op.into())
    }

    fn format(&self, num: i128) -> anyhow::Result<String, NumberFormatterError> {
        Ok(format!("{:x}", &num))
    }
}

struct BinaryNumberFormatter;
impl NumberFormatter for BinaryNumberFormatter {
    fn read(&self, binary_num: &str) -> anyhow::Result<i128, NumberFormatterError> {
        let without_prefix = binary_num.trim_start_matches("0b");
        i128::from_str_radix(without_prefix, 2).map_err(|op| op.into())
    }

    fn format(&self, num: i128) -> anyhow::Result<String, NumberFormatterError> {
        Ok(format!("{:b}", num))
    }
}

struct NumberFormatterFactory;
impl NumberFormatterFactory {
    pub fn new_number_formatter(number_type: &NumberType) -> Box<dyn NumberFormatter> {
        match number_type {
            NumberType::Integer => Box::new(IntegerNumberFormatter {}),
            NumberType::Hexadecimal => Box::new(HexadecimalNumberFormatter {}),
            NumberType::Binary => Box::new(BinaryNumberFormatter {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn new_command_options_should_return_expected_value() {
        // Arrange
        let expected = CommandOptions {
            number_type: NumberType::Binary,
            number: String::from("input"),
        };

        // Act
        let actual = CommandOptions::new(expected.number_type, &expected.number);

        // Assert
        assert_eq!(actual.number_type, expected.number_type);
        assert_eq!(actual.number, expected.number);
    }

    #[test_case(NumberType::Integer)]
    #[test_case(NumberType::Hexadecimal)]
    #[test_case(NumberType::Binary)]
    fn new_number_formatter_should_match_number_type(number_type: NumberType) {
        let _ = *NumberFormatterFactory::new_number_formatter(&number_type);
    }

    #[test_case(NumberType::Integer, "10", 10)]
    #[test_case(NumberType::Hexadecimal, "FFFF", 65535)]
    #[test_case(NumberType::Binary, "0000110", 6)]
    fn new_number_formatter_should_read_number_type(
        number_type: NumberType,
        input_number: &str,
        expected_number: i128,
    ) {
        // Arrange
        let reader = NumberFormatterFactory::new_number_formatter(&number_type);

        // Act
        let actual_number = reader.read(input_number);

        // Assert
        assert!(actual_number.is_ok());
        assert_eq!(expected_number, actual_number.unwrap());
    }

    #[test_case(NumberType::Integer, 907823, "907823")]
    #[test_case(NumberType::Hexadecimal, 65451, "ffab")]
    #[test_case(NumberType::Binary, 9543, "10010101000111")]
    fn new_number_formatter_should_format_i128(
        number_type: NumberType,
        input_number: i128,
        expected_output: &str,
    ) {
        // Arrange
        let reader = NumberFormatterFactory::new_number_formatter(&number_type);

        // Act
        let actual_number = reader.format(input_number);

        // Assert
        assert!(actual_number.is_ok());
        assert_eq!(expected_output, actual_number.unwrap());
    }

    #[test]
    fn command_context_should_format_all_types_correctly() {
        // Arrange
        let command_options = CommandOptions {
            number_type: NumberType::Binary,
            number: "0b1101011".to_string(),
        };

        let command_context = CommandContext::new(command_options);

        // Act
        let output = command_context.format_all_number_types();

        // Assert
        assert!(output.is_ok());
        let vec = output.unwrap();

        assert!(vec.contains(&(NumberType::Integer, "107".to_string())));
        assert!(vec.contains(&(NumberType::Binary, "1101011".to_string())));
        assert!(vec.contains(&(NumberType::Hexadecimal, "6b".to_string())));
    }

    #[test_case(CommandOptions { number_type: NumberType::Integer, number: "12".to_string() })]
    #[test_case(CommandOptions { number_type: NumberType::Binary, number: "100001".to_string() })]
    #[test_case(CommandOptions { number_type: NumberType::Hexadecimal, number: "0xAbC3f09".to_string() })]
    fn command_context_should_format_all_types_in_expected_order(command_options: CommandOptions) {
        // Arrange
        let command_context = CommandContext::new(command_options);

        // Act
        let output = command_context.format_all_number_types();

        // Assert
        assert!(output.is_ok());
        let vec = output.unwrap();

        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0].0, NumberType::Integer);
        assert_eq!(vec[1].0, NumberType::Hexadecimal);
        assert_eq!(vec[2].0, NumberType::Binary);
    }
}
