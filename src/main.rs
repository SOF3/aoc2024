use std::path::PathBuf;
use std::time::Instant;
use std::{env, fmt, fs, io};

use anyhow::Context;
use clap::{Parser, ValueEnum};

mod d1;

macro_rules! main {
    (
        $args:ident;
        $(day $day:literal part $part:literal: $impls:tt;)*
    ) => {
        let variant = $args.variant.as_str();
        match ($args.day, $args.part) {
            $(
                ($day, $part) => {
                    let input = load_input($args.mode, $day)?;

                    let start_time = Instant::now();
                    let output = main!(@impl variant, &input, $impls);
                    let end_time = Instant::now();
                    eprintln!("Execution time: {}ns", (end_time - start_time).as_nanos());

                    println!("{output}");
                }
            )*
        _ => anyhow::bail!("Unimplemented day/part"),
        }
    };
    (@impl $variant:ident, $input:expr, {
        _ => $path:path,
    }) => {
        call($path, $input)
    };
    (@impl $variant:ident, $input:expr, {
        $($name:literal => $path:path,)*
    }) => {
        match $variant {
            $($name => call($path, $input),)*
            _ => anyhow::bail!("Unknown variant implementation"),
        }
    };
}

fn call<Out: fmt::Display>(f: impl Fn(&str) -> Out, input: &str) -> Out { f(input) }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Sample,
    Private,
}

fn main() -> anyhow::Result<()> {
    #[derive(Parser)]
    struct Args {
        mode:    Mode,
        day:     u32,
        part:    u32,
        #[clap(default_value = "")]
        variant: String,
    }

    let args = Args::parse();

    main! {
        args;
        day 1 part 1: {
            _ => d1::p1,
        };
        day 1 part 2: {
            "sorted" => d1::p2_sorted,
        };
    }

    Ok(())
}

fn load_input(mode: Mode, day: u32) -> anyhow::Result<String> {
    let dir = env::var("CARGO_MANIFEST_DIR").context("need cargo run")?;
    let path = PathBuf::from(dir).join("input").join(format!(
        "d{day}.{}.input.txt",
        match mode {
            Mode::Sample => "sample",
            Mode::Private => "private",
        }
    ));

    if let Mode::Private = mode {
        let exists =
            fs::exists(&path).with_context(|| format!("test {} existence", path.display()))?;
        if !exists {
            eprintln!("Downloading day {day} input");

            let session_cookie = env::var("AOC_SESSION")
                .context("private file missing and AOC_SESSION env var missing")?;

            let client = reqwest::blocking::Client::new();
            let data = client
                .get(format!("https://adventofcode.com/2024/day/{day}/input"))
                .header("Cookie", format!("session={session_cookie}"))
                .send()
                .context("request aoc private input")?;
            let input = io::read_to_string(data).context("read aoc private input")?;
            fs::write(&path, &input).context("write aoc private input to cache")?;
        }
    }

    fs::read_to_string(&path).with_context(|| format!("read file {}", path.display()))
}
