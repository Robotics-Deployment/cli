mod initialize;
use initialize::init::init;

mod build;
use build::build::build;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    init: Option<String>,

    #[arg(short, long)]
    build: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(_) = args.init {
        init();
    } else if let Some(_) = args.build {
        build();
    }
}