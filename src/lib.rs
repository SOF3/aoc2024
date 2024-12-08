use std::path::PathBuf;
use std::{env, fs, io};

use anyhow::Context;
use clap::{Parser, ValueEnum};

pub mod all;
pub use all::run;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Sample,
    Private,
}

#[derive(Parser)]
pub struct Args {
    mode:    Mode,
    day:     u32,
    part:    u32,
    #[clap(default_value = "")]
    variant: String,
}

pub fn load_input(mode: Mode, day: u32) -> anyhow::Result<String> {
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

pub trait Parse: Clone {
    fn parse(input: &str) -> Self;
}

impl Parse for String {
    fn parse(input: &str) -> Self { input.to_string() }
}

#[derive(Clone)]
pub struct JsonString(pub String);

impl Parse for JsonString {
    fn parse(input: &str) -> Self {
        let value = simd_json::json!(input);
        Self(simd_json::to_string(&value).unwrap())
    }
}

mod util;
