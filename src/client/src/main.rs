mod initialize;
use initialize::init::init;

mod build;
use build::build::build;

mod verify;
use verify::verify::verify;

mod test;
use test::test::test;

use clap::Parser;

#[derive(Parser, Debug)]
enum Command {
    Init,
    Build,
    Verify,
    Test,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    verify: bool,

    #[clap(short, long)]
    init: bool,

    #[clap(short, long)]
    build: bool,

    #[clap(short, long)]
    test: bool,
}

impl Args {
    fn command(&self) -> Option<Command> {
        if self.init {
            Some(Command::Init)
        } else if self.build {
            Some(Command::Build)
        } else if self.verify {
            Some(Command::Verify)
        } else if self.test {
            Some(Command::Test)
        } else {
            None
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.command() {
        Some(Command::Init) => init(),
        Some(Command::Build) => build(),
        Some(Command::Verify) => verify(),
        Some(Command::Test) => test(),
        None => println!("No command specified."),
    }
}
