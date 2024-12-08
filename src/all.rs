use std::fmt;
use std::time::Instant;

use crate::{load_input, Args, JsonString, Parse};

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
            "ticked-boolvec" => p1_ticked_boolvec,
            "ticked-bitvec" => p1_ticked_bitvec,
        }
        part 2 {
            "brute-fxhash-loc" => p2_brute_fxhash_loc,
        }
    }
    day 7 {
        part 1 {
            "reversed" => p1_reversed,
            "jq" => jq["d7q1"],
        }
        part 2 {
            "reversed" => p2_reversed,
            "jq" => jq["d7q2"],
        }
    }
    day 8 {
        part 1 {
            "naive" => p1_naive,
        }
        part 2 {
            "naive" => p2_naive,
        }
    }
}
