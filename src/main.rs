use clap::StructOpt;
use format_number::{CommandContext, CommandOptions, NumberFormatterError};

fn main() -> anyhow::Result<(), NumberFormatterError> {
    let command_context = CommandContext::new(CommandOptions::parse());

    let result = command_context.format_all_number_types()?;
    for (number_type, output) in result {
        println!("{}: {}", number_type, output);
    }

    Ok(())
}
