use clap::Parser;

/// A simple DSL which allows you to create data simple and fast
#[derive(Parser,Debug)]
#[clap(author = "Caviar-X",version = "0.0.1",about,long_about = None)]
pub struct Args {
    /// The generate times
    #[clap(short, long, default_value_t = 10)]
    count : u8,
    /// The name of the file
    #[clap(short, long)]
    file : String
}
