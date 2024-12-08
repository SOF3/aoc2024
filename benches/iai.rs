#[cfg(feature = "iai-bench")]
aoc2024::iai!();

#[cfg(not(feature = "iai-bench"))]
fn main() {
    println!("iai bench skipped due to disabled feature");
}
