use std::fmt;

use aoc2024::*;
use criterion::*;

fn bench(criterion_manager: &mut Criterion) {
    fn run(criterion_manager: &mut Criterion) -> anyhow::Result<()> {
        aoc2024::bench!(criterion_manager);
        Ok(())
    }

    run(criterion_manager).unwrap();
}

fn call_benched<In: Parse, Out: fmt::Display>(b: &mut Bencher, day: u32, f: impl FnMut(In) -> Out) {
    let input = load_input(Mode::Private, day).unwrap();
    let parsed: In = Parse::parse(&input);
    b.iter_batched(|| parsed.clone(), f, BatchSize::LargeInput);
}

criterion_group!(benches, bench);
criterion_main!(benches);
