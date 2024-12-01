use std::path::PathBuf;
use std::time::Instant;
use std::{env, fmt, fs, io};

use anyhow::Context;
use clap::{Parser, ValueEnum};
use criterion::{BatchSize, Criterion, Bencher};

macro_rules! main {
    (
        $(day $day:literal {
            $(part $part:literal $impls:tt)*
        })*
    ) => {
        $(
            paste::paste! {
                mod [<d $day>];
            }
        )*

        pub fn run(args: Args) -> anyhow::Result<()> {
            let variant = args.variant.as_str();
            match args.day {
                $(
                    $day => match args.part {
                        $(
                            $part => {
                                let input = load_input(args.mode, $day)?;

                                let output = main!(@impl variant, &input, $impls);

                                println!("{output}");

                                Ok(())
                            },
                        )*
                        _ => anyhow::bail!("Unimplemented part"),
                    },
                )*
                _ => anyhow::bail!("Unimplemented day"),
            }
        }

        #[allow(dead_code)]
        pub fn bench(c: &mut Criterion) {
            fn try_unwrap<R, E: fmt::Debug>(f: impl FnOnce() -> Result<R, E>) -> R {
                f().unwrap()
            }

            $($(
                {
                    let mut group = c.benchmark_group(concat!("Day ", $day, " Part ", $part));
                    main!(@bench group, $day, $impls);
                    group.finish();
                }
            )*)*
        }
    };
    (@impl $variant:ident, $input:expr, {
        $($name:literal => $fn:expr,)*
    }) => {
        match $variant {
            $($name => call($fn, $input),)*
            _ => anyhow::bail!("Unknown variant implementation"),
        }
    };
    (@bench $group:ident, $day:literal, {
        $($name:literal => $fn:expr,)*
    }) => {
        $(
            {
                let mut f = try_unwrap(move || anyhow::Ok($fn));
                $group.bench_function($name, move |b| {
                    call_benched(b, $day, &mut f);
                });
            }
        )*
    };
}

macro_rules! jq {
    ($file:literal, $function:literal) => {{
        let mut program =
            jq_rs::compile(concat!(include_str!(concat!("all/", $file)), "\n", $function))
                .map_err(|err| anyhow::anyhow!("compile {}: {err}", $file))?;
        move |data: JsonString| -> String {
            let output = program.run(data.0.as_str()).expect("jq program error");
            output
        }
    }};
}

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

#[allow(dead_code)]
fn call_benched<In: Parse, Out: fmt::Display>(b: &mut Bencher, day: u32, f: impl FnMut(In) -> Out) {
    let input = load_input(Mode::Private, day).unwrap();
    let parsed: In = Parse::parse(&input);
    b.iter_batched(
        || parsed.clone(),
        f,
        BatchSize::LargeInput,
    );
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
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

main! {
    day 1 {
        part 1 {
            "zip" => d1::p1_zip,
            "jq" => jq!("d1.jq", "d1q1"),
        }
        part 2 {
            "hash" => d1::p2_hash,
            "sorted" => d1::p2_sorted,
            "count" => d1::p2_count,
            "bitvec" => d1::p2_bitvec,
            "jq/hash" => jq!("d1.jq", "d1q2_hash"),
        }
    }
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

pub trait Parse: Clone {
    fn parse(input: &str) -> Self;
}

impl Parse for String {
    fn parse(input: &str) -> Self { input.to_string() }
}

#[derive(Clone)]
struct JsonString(String);

impl Parse for JsonString {
    fn parse(input: &str) -> Self {
        let value = simd_json::json!(input);
        Self(simd_json::to_string(&value).unwrap())
    }
}
