use clap::Parser;

mod all;
use all::Parse;

fn main() -> anyhow::Result<()> {
    let args = all::Args::parse();

    all::run(args)
}
