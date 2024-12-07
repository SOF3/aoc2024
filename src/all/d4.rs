use std::array;

use crate::util::{DirectDiagonal, GridView};

pub fn p1_brute(input: String) -> u32 {
    let grid = GridView::new(&input);

    let mut count = 0;

    for (index, _) in input.match_indices('X') {
        let loc = grid.index_to_loc(index).unwrap();
        for dir in DirectDiagonal::ALL {
            let mut iter =
                loc.direct_diagonal_iter(dir, grid).skip(1).take(3).map(|loc| grid.get(loc));
            let chars: [_; 3] = array::from_fn(|_| iter.next().flatten());
            if chars == [Some(b'M'), Some(b'A'), Some(b'S')] {
                count += 1;
            }
        }
    }

    count
}
