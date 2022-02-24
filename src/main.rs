use clap::StructOpt;
use format_number::CommandOptions;

fn main() {
    let cli = CommandOptions::parse();
    println!("{:?}", cli);
}
