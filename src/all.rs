use std::path::PathBuf;
use std::time::Instant;
use std::{env, fmt, fs, io};

use anyhow::Context;
use clap::{Parser, ValueEnum};

fn call<In: Parse, Out: fmt::Display>(mut f: impl FnMut(In) -> Out, input: &str) -> String {
    let start_time = Instant::now();
    let parsed = Parse::parse(input);
    let duration = Instant::now() - start_time;
    eprintln!("Parse time: {}ms", duration.as_secs_f32() * 1000.);

    let start_time = Instant::now();
    let output = f(parsed);
    let duration = Instant::now() - start_time;
    eprintln!("Execution time: {}ms", duration.as_secs_f32() * 1000.);

    output.to_string()
}

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

macros::all! {
    day 1 {
        part 1 {
            "zip" => p1_zip,
            "jq" => jq["d1q1"],
        }
        part 2 {
            "hash" => p2_hash,
            "sorted" => p2_sorted,
            "count" => p2_count,
            "bitvec" => p2_bitvec,
            "jq/hash" => jq["d1q2_hash"],
        }
    }
    day 2 {
        part 1 {
            "windows" => p1_windows,
            "first-all" => p1_first_all,
            "jq" => jq["d2q1"],
        }
        part 2 {
            "brute" => p2_brute_force,
            "vec" => p2_vec,
            "jq" => jq["d2q2"],
        }
    }
    day 3 {
        part 1 {
            "find" => p1_find,
            "jq" => jq["d3q1"],
        }
        part 2 {
            "find" => p2_find,
            "jq" => jq["d3q2"],
        }
    }
    day 4 {
        part 1 {
            "brute" => p1_brute,
        }
        part 2 {
            "brute" => p2_brute,
        }
    }
    day 5 {
        part 1 {
            "fxhashmap-fxhashset" => p1_fxhashmap_fxhashset,
            "btreemap-fxhashset" => p1_btreemap_fxhashset,
            "fxhashmap-vec" => p1_fxhashmap_vec,
            "btreemap-vec" => p1_btreemap_vec,
        }
    }
    day 6 {
        part 1 {
            "ticked-fxhash-loc" => p1_ticked_fxhash_loc,
            "ticked-fxhash-index" => p1_ticked_fxhash_index,
            "ticked-bitvec" => p1_ticked_bitvec,
        }
    }
}
