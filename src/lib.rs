use clap::{ArgEnum, Parser};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(ArgEnum, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberType {
    Integer,
    Hexadecimal,
    Binary,
}

#[derive(Debug, Parser)]
#[clap(name = "format-number")]
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
}

#[derive(Error, Debug)]
enum NumberFormatterError {
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
        i128::from_str_radix(binary_num, 2).map_err(|op| op.into())
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
}
