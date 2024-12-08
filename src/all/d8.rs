use rustc_hash::FxHashMap;

use crate::util::{GridLoc, GridView};

fn disjoint_pairs<T>(slice: &[T]) -> impl Iterator<Item = [&T; 2]> {
    (0..slice.len()).flat_map(move |left| {
        (left + 1..slice.len()).map(move |right| [&slice[left], &slice[right]])
    })
}

pub fn p1_naive(input: String) -> u32 {
    let grid = GridView::new(input.into_bytes());

    let mut freqs = FxHashMap::<u8, Vec<u32>>::default();
    for (index, &ch) in grid.input.iter().enumerate() {
        if ch != b'.' && ch != b'\n' {
            freqs.entry(ch).or_default().push(index as u32);
        }
    }

    let mut matches = 0;
    for indices in freqs.values() {
        for pair_index in disjoint_pairs(indices) {
            let pair_loc =
                pair_index.map(|&index| grid.shape.index_to_loc(index as usize).unwrap());
            let vector = pair_loc[1] - pair_loc[0];

            for loc in [pair_loc[0].add(vector * -1, &grid), pair_loc[1].add(vector, &grid)]
                .into_iter()
                .flatten()
            {
                let byte = grid.get(loc).unwrap();
                if byte == b'.' {
                    matches += 1;
                }
            }
        }
    }

    // turns out it is the number of location-frequency pairs not unique locations???
    matches
}

pub fn p2_naive(input: String) -> u32 {
    let grid = GridView::new(input.into_bytes());

    let mut freqs = FxHashMap::<u8, Vec<u32>>::default();
    for (index, &ch) in grid.input.iter().enumerate() {
        if ch != b'.' && ch != b'\n' {
            freqs.entry(ch).or_default().push(index as u32);
        }
    }

    let mut output = vec![false; grid.input.len()];

    for indices in freqs.values() {
        for pair_index in disjoint_pairs(indices) {
            let pair_loc =
                pair_index.map(|&index| grid.shape.index_to_loc(index as usize).unwrap());
            let vector = pair_loc[1] - pair_loc[0];

            let iter_map_fn_builder = {
                let shape = grid.shape;
                move |loc: GridLoc, factor: i32| move |step| loc.add(vector * step * factor, shape)
            };

            for map_fn in
                [iter_map_fn_builder(pair_loc[0], -1), iter_map_fn_builder(pair_loc[1], 1)]
            {
                let mut loc_iter = (0..).map(map_fn);

                while let Some(Some(loc)) = loc_iter.next() {
                    output[grid.shape.loc_to_index(loc) as usize] = true;
                }
            }
        }
    }

    output.iter().map(|&b| if b { 1 } else { 0 }).sum()
}
