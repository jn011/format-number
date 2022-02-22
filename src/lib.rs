use std::num::ParseIntError;

use thiserror::Error;

pub struct CommandOptions {
    pub input_argument: String,
    pub input: String,
    pub output_argument: String,
}

impl CommandOptions {
    pub fn new(input_argument: &str, input: &str, output_argument: &str) -> Self {
        Self {
            input_argument: input_argument.to_string(),
            input: input.to_string(),
            output_argument: output_argument.to_string(),
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

enum NumberType {
    Integer,
    Hexadecimal,
    Binary,
}

#[derive(Error, Debug)]
enum NumberReaderError {
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

impl From<ParseIntError> for NumberReaderError {
    fn from(e: ParseIntError) -> Self {
        match &e.kind() {
            std::num::IntErrorKind::Empty => NumberReaderError::Empty,
            std::num::IntErrorKind::InvalidDigit => NumberReaderError::InvalidDigit,
            std::num::IntErrorKind::PosOverflow => NumberReaderError::TooLargeError,
            std::num::IntErrorKind::NegOverflow => NumberReaderError::TooSmallError,
            _ => NumberReaderError::Unknown,
        }
    }
}

trait NumberReader {
    fn read(&self, num: &str) -> anyhow::Result<i128, NumberReaderError>;
}

struct IntegerNumberReader;
impl NumberReader for IntegerNumberReader {
    fn read(&self, integer: &str) -> anyhow::Result<i128, NumberReaderError> {
        integer.parse::<i128>().map_err(|op| op.into())
    }
}

struct HexadecimalNumberReader;
impl NumberReader for HexadecimalNumberReader {
    fn read(&self, hexadecimal: &str) -> anyhow::Result<i128, NumberReaderError> {
        let without_prefix = hexadecimal.trim_start_matches("0x");
        i128::from_str_radix(without_prefix, 16).map_err(|op| op.into())
    }
}

struct BinaryNumberReader;
impl NumberReader for BinaryNumberReader {
    fn read(&self, binary_num: &str) -> anyhow::Result<i128, NumberReaderError> {
        i128::from_str_radix(binary_num, 2).map_err(|op| op.into())
    }
}

struct NumberReaderFactory;
impl NumberReaderFactory {
    pub fn new_number_reader(number_type: &NumberType) -> Box<dyn NumberReader> {
        match number_type {
            NumberType::Integer => Box::new(IntegerNumberReader {}),
            NumberType::Hexadecimal => Box::new(HexadecimalNumberReader {}),
            NumberType::Binary => Box::new(BinaryNumberReader {}),
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
            input_argument: String::from("input_argument"),
            input: String::from("input"),
            output_argument: String::from("output_argument"),
        };

        // Act
        let actual = CommandOptions::new(
            &expected.input_argument,
            &expected.input,
            &expected.output_argument,
        );

        // Assert
        assert_eq!(actual.input_argument, expected.input_argument);
        assert_eq!(actual.input, expected.input);
        assert_eq!(actual.output_argument, expected.output_argument);
    }

    #[test_case(NumberType::Integer)]
    #[test_case(NumberType::Hexadecimal)]
    #[test_case(NumberType::Binary)]
    fn new_number_reader_should_match_number_type(number_type: NumberType) {
        let _ = *NumberReaderFactory::new_number_reader(&number_type);
    }

    #[test_case(NumberType::Integer, "10", 10)]
    #[test_case(NumberType::Hexadecimal, "FFFF", 65535)]
    #[test_case(NumberType::Binary, "0000110", 6)]
    fn new_number_reader_should_parse_number_type(
        number_type: NumberType,
        input_number: &str,
        expected_number: i128,
    ) {
        // Arrange
        let reader = NumberReaderFactory::new_number_reader(&number_type);

        // Act
        let actual_number = reader.read(input_number);

        // Assert
        assert!(actual_number.is_ok());
        assert_eq!(expected_number, actual_number.unwrap());
    }
}
