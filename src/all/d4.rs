use std::array;

use crate::util::{Direct, DirectBoth, DirectDiagonal, GridView};

pub fn p1_brute(input: String) -> u32 {
    let grid = GridView::new(&input);

    let mut count = 0;

    for (index, _) in input.match_indices('X') {
        let loc = grid.index_to_loc(index).unwrap();
        for &dir in DirectBoth::ALL {
            let mut iter = loc.direct_iter(dir, &grid).skip(1).take(3).map(|loc| grid.get(loc));
            let chars: [_; 3] = array::from_fn(|_| iter.next().flatten());
            if chars == [Some(b'M'), Some(b'A'), Some(b'S')] {
                count += 1;
            }
        }
    }

    count
}

pub fn p2_brute(input: String) -> u32 {
    let grid = GridView::new(&input);

    let mut count = 0;

    for (index, _) in input.match_indices('A') {
        let loc = grid.index_to_loc(index).unwrap();
        let matched = [
            [DirectDiagonal::LeftUp, DirectDiagonal::RightDown],
            [DirectDiagonal::RightUp, DirectDiagonal::LeftDown],
        ]
        .map(|ends| {
            let values = ends.map(|direct| grid.get(loc.direct(direct, &grid)?));
            values == [Some(b'M'), Some(b'S')] || values == [Some(b'S'), Some(b'M')]
        });
        if matched[0] && matched[1] {
            count += 1;
        }
    }

    count
}
