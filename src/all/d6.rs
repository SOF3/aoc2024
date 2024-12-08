use std::collections::HashSet;
use std::hash::BuildHasher;

use bitvec::vec::BitVec;
use rustc_hash::FxHashSet;

use crate::util::{DirectTaxicab, GridLoc, GridView};

trait LocCounter {
    fn new(capacity: usize) -> Self;
    fn insert(&mut self, loc: impl FnOnce() -> GridLoc, index: impl FnOnce() -> u32);
    fn count(&self) -> u32;
}

fn p1_ticked<CollectorT: LocCounter>(input: String) -> u32 {
    let grid = GridView::new(&input);

    let mut loc = grid.shape.index_to_loc(input.find('^').unwrap()).unwrap();
    let mut direct = DirectTaxicab::Up;

    let mut collector = CollectorT::new(input.len());
    'ticks: loop {
        collector.insert(|| loc, || grid.shape.loc_to_index(loc));

        'directs: loop {
            match loc.direct(direct, &grid) {
                None => return collector.count(), // leave map
                Some(new_loc) => match grid.get(new_loc).unwrap() {
                    b'^' | b'.' => {
                        loc = new_loc;
                        continue 'ticks;
                    }
                    b'#' => {
                        direct = direct.clockwise();
                        continue 'directs;
                    }
                    _ => unreachable!(),
                },
            }
        }
    }
}

impl<S: BuildHasher + Default> LocCounter for HashSet<GridLoc, S> {
    fn new(capacity: usize) -> Self { Self::with_capacity_and_hasher(capacity, S::default()) }

    fn insert(&mut self, loc: impl FnOnce() -> GridLoc, _: impl FnOnce() -> u32) {
        HashSet::insert(self, loc());
    }

    fn count(&self) -> u32 { self.len() as u32 }
}

impl<S: BuildHasher + Default> LocCounter for HashSet<u32, S> {
    fn new(capacity: usize) -> Self { Self::with_capacity_and_hasher(capacity, S::default()) }

    fn insert(&mut self, _: impl FnOnce() -> GridLoc, index: impl FnOnce() -> u32) {
        HashSet::insert(self, index());
    }

    fn count(&self) -> u32 { self.len() as u32 }
}

impl LocCounter for Vec<bool> {
    fn new(capacity: usize) -> Self { vec![false; capacity] }

    fn insert(&mut self, _: impl FnOnce() -> GridLoc, index: impl FnOnce() -> u32) {
        self[index() as usize] = true;
    }

    fn count(&self) -> u32 {
        let mut output = 0;
        // we don't use Iterator::count() here because it uses usize instead of u32
        for &b in self {
            if b {
                output += 1;
            }
        }
        output
    }
}

impl LocCounter for BitVec {
    fn new(capacity: usize) -> Self { Self::repeat(false, capacity) }

    fn insert(&mut self, _: impl FnOnce() -> GridLoc, index: impl FnOnce() -> u32) {
        self.set(index() as usize, true);
    }

    fn count(&self) -> u32 { self.count_ones() as u32 }
}

pub fn p1_ticked_fxhash_loc(input: String) -> u32 { p1_ticked::<FxHashSet<GridLoc>>(input) }
pub fn p1_ticked_fxhash_index(input: String) -> u32 { p1_ticked::<FxHashSet<u32>>(input) }
pub fn p1_ticked_boolvec(input: String) -> u32 { p1_ticked::<Vec<bool>>(input) }
pub fn p1_ticked_bitvec(input: String) -> u32 { p1_ticked::<BitVec>(input) }

trait LoopDetector {
    fn new(capacity: usize) -> Self;
    fn clear(&mut self);

    fn insert(
        &mut self,
        loc: impl FnOnce() -> GridLoc,
        index: impl FnOnce() -> u32,
        direct: DirectTaxicab,
    ) -> IsLooped;
}

#[derive(PartialEq, Eq)]
enum IsLooped {
    Repeating,
    NewStep,
}

fn is_looping<DetectorT: LoopDetector>(
    grid: &GridView<impl AsRef<[u8]>>,
    det: &mut DetectorT,
    initial: GridLoc,
) -> bool {
    det.clear();

    let mut loc = initial;
    let mut direct = DirectTaxicab::Up;

    'ticks: loop {
        if det.insert(|| loc, || grid.shape.loc_to_index(loc), direct) == IsLooped::Repeating {
            return true;
        }

        'directs: loop {
            match loc.direct(direct, grid) {
                None => return false, // leave map
                Some(new_loc) => match grid.get(new_loc).unwrap() {
                    b'^' | b'.' => {
                        loc = new_loc;
                        continue 'ticks;
                    }
                    b'#' => {
                        direct = direct.clockwise();
                        continue 'directs;
                    }
                    _ => unreachable!(),
                },
            }
        }
    }
}

fn p2_brute<LoopDetectorT: LoopDetector>(input: String) -> u32 {
    let initial_index = input.find('^').unwrap();
    let size = input.len();
    let mut det = LoopDetectorT::new(size);

    let input = input.into_bytes();
    let mut grid = GridView::new(input);
    let initial = grid.shape.index_to_loc(initial_index).unwrap();

    let mut count = 0;
    for index in 0..size {
        if grid.input[index] == b'.' {
            grid.input[index] = b'#';
            if is_looping(&grid, &mut det, initial) {
                count += 1;
            }
            grid.input[index] = b'.';
        }
    }
    count
}

impl<S: BuildHasher + Default> LoopDetector for HashSet<(GridLoc, DirectTaxicab), S> {
    fn new(capacity: usize) -> Self { HashSet::with_capacity_and_hasher(capacity, S::default()) }

    fn clear(&mut self) { HashSet::clear(self) }

    fn insert(
        &mut self,
        loc: impl FnOnce() -> GridLoc,
        _: impl FnOnce() -> u32,
        direct: DirectTaxicab,
    ) -> IsLooped {
        if HashSet::insert(self, (loc(), direct)) {
            IsLooped::NewStep
        } else {
            IsLooped::Repeating
        }
    }
}

pub fn p2_brute_fxhash_loc(input: String) -> u32 {
    p2_brute::<FxHashSet<(GridLoc, DirectTaxicab)>>(input)
}
