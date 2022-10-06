use clap::{command, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    server: String,

    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    password: String,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() {
    let _cli = Args::parse();
}
