use std::{collections::HashSet, hash::BuildHasher};

use bitvec::vec::BitVec;
use rustc_hash::FxHashSet;

use crate::util::{DirectTaxicab, GridLoc, GridView};

trait Collector {
    fn new(capacity: usize) -> Self;
    fn insert(&mut self, loc: impl FnOnce() -> GridLoc, index: impl FnOnce() -> u32);
    fn count(&self) -> u32;
}

fn p1_ticked<CollectorT: Collector>(input: String) -> u32 {
    let grid = GridView::new(&input);

    let mut loc = grid.index_to_loc(input.find('^').unwrap()).unwrap();
    let mut direct = DirectTaxicab::Up;

    let mut collector = CollectorT::new(input.len());
    'ticks: loop {
        collector.insert(|| loc, || grid.loc_to_index(loc));

        'directs: loop {
            match loc.direct(direct, grid) {
                None => return collector.count(), // leave map
                Some(new_loc) => {
                    match grid.get(new_loc).unwrap() {
                        b'^' | b'.' => {
                            loc = new_loc;
                            continue 'ticks
                        }
                        b'#' => {
                            direct = direct.clockwise();
                            continue 'directs
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

impl<S: BuildHasher + Default> Collector for HashSet<GridLoc, S> {
    fn new(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, S::default())
    }

    fn insert(&mut self, loc: impl FnOnce() -> GridLoc, _: impl FnOnce() -> u32) {
        HashSet::insert(self, loc());
    }

    fn count(&self) -> u32 {
        self.len() as u32
    }
}

impl<S: BuildHasher + Default> Collector for HashSet<u32, S> {
    fn new(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, S::default())
    }

    fn insert(&mut self, _: impl FnOnce() -> GridLoc, index: impl FnOnce() -> u32) {
        HashSet::insert(self, index());
    }

    fn count(&self) -> u32 {
        self.len() as u32
    }
}

impl Collector for BitVec {
    fn new(capacity: usize) -> Self {
        Self::repeat(false, capacity)
    }

    fn insert(&mut self, _: impl FnOnce() -> GridLoc, index: impl FnOnce() -> u32) {
        self.set(index() as usize, true);
    }

    fn count(&self) -> u32 {
        self.count_ones() as u32
    }
}

pub fn p1_ticked_fxhash_loc(input: String) -> u32 { p1_ticked::<FxHashSet<GridLoc>>(input) }
pub fn p1_ticked_fxhash_index(input: String) -> u32 { p1_ticked::<FxHashSet<u32>>(input) }
pub fn p1_ticked_bitvec(input: String) -> u32 { p1_ticked::<BitVec>(input) }
