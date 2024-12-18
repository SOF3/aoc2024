use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = aoc2024::Args::parse();
    aoc2024::run(args)
}
