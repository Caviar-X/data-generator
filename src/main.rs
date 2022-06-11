use clap::Parser as ClapParser;
use colorful::*;
use data_generator::parser::Parser;
use std::fs::File;
use std::io::Write;
/// A simple DSL which allows you to create data simple and fast
#[derive(ClapParser, Debug, Clone)]
#[clap(author = "Caviar-X",version = "1.0.0",about,long_about = None)]
pub struct Args {
    /// The generate times
    #[clap(short, long, default_value_t = 10)]
    pub count: u8,
    /// The name of the file
    #[clap(short, long)]
    pub file: String,
}
fn main() {
    let params: Args = Args::parse();
    for i in 1..=params.count {
        let mut data: File = File::create(format!("{}.in", i)).expect("Failed to create file");
        let mut parser: Parser = Parser::new(params.clone().file);
        loop {
            let clone = parser.clone();
            let token: Option<Vec<&str>> = parser.next_token();
            if token == None {
                break;
            }
            if let Some(t) = token {
                write!(data, "{} ", clone.parse(t).unwrap()).expect("Failed to write data");
            }
        }
        eprintln!("{}", format!("Generating file {}.in", i).color(Color::Blue));
    }
}
