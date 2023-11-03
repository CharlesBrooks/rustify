use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,
}

fn main() {
    let _cli = Cli::parse();
    println!("Hello, world!");
}
