use std::collections::{BTreeSet, HashSet};
use std::hash::BuildHasher;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::Parse;

#[derive(Clone)]
pub struct Input {
    constraints: Vec<Constraint>,
    updates:     Vec<Update>,
}

#[derive(Clone)]
struct Constraint {
    earlier: u32,
    later:   u32,
}

#[derive(Clone)]
struct Update(Vec<u32>);

impl Parse for Input {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let mut constraints = Vec::new();
        let mut updates = Vec::new();

        for line in lines.by_ref() {
            if line == "" {
                break;
            }

            let (left, right) = line.split_once('|').unwrap();
            constraints.push(Constraint {
                earlier: left.parse().unwrap(),
                later:   right.parse().unwrap(),
            });
        }

        for line in lines {
            if line == "" {
                break;
            }

            updates.push(Update(line.split(',').map(|s| s.parse().unwrap()).collect()));
        }

        Self { constraints, updates }
    }
}

trait DenyLists: Default {
    fn insert(&mut self, later: u32, earlier: u32);

    fn get_earlier_items(&self, later: u32) -> impl Iterator<Item = u32>;
}

#[derive(Default)]
struct FxHashMapDenyLists(FxHashMap<u32, Vec<u32>>);

impl DenyLists for FxHashMapDenyLists {
    fn insert(&mut self, later: u32, earlier: u32) {
        self.0.entry(later).or_default().push(earlier);
    }

    fn get_earlier_items(&self, later: u32) -> impl Iterator<Item = u32> {
        self.0.get(&later).into_iter().flatten().copied()
    }
}

#[derive(Default)]
struct BTreeSetDenyLists(BTreeSet<(u32, u32)>);

impl DenyLists for BTreeSetDenyLists {
    fn insert(&mut self, later: u32, earlier: u32) { self.0.insert((later, earlier)); }

    fn get_earlier_items(&self, later: u32) -> impl Iterator<Item = u32> {
        self.0.range((later, 0)..=(later, u32::MAX)).map(|&(_, earlier)| earlier)
    }
}

trait DisallowedSet: Default + Extend<u32> {
    fn contains(&self, item: u32) -> bool;

    fn clear(&mut self);
}

impl<S: BuildHasher + Default> DisallowedSet for HashSet<u32, S> {
    fn contains(&self, item: u32) -> bool { HashSet::contains(self, &item) }

    fn clear(&mut self) { HashSet::clear(self); }
}

impl DisallowedSet for Vec<u32> {
    fn contains(&self, item: u32) -> bool { self.iter().any(|&v| v == item) }

    fn clear(&mut self) { Vec::clear(self); }
}

fn p1<DenyListsT: DenyLists, DisallowedSetT: DisallowedSet>(input: Input) -> u32 {
    let mut deny_lists = DenyListsT::default();

    for constraint in input.constraints {
        deny_lists.insert(constraint.later, constraint.earlier);
    }

    let mut result = 0;

    let mut disallowed = DisallowedSetT::default();

    'update: for Update(items) in input.updates {
        disallowed.clear();

        for &item in &items {
            if disallowed.contains(item) {
                continue 'update;
            }

            disallowed.extend(deny_lists.get_earlier_items(item));
        }

        result += items[items.len() / 2];
    }

    result
}

pub fn p1_fxhashmap_fxhashset(input: Input) -> u32 {
    p1::<FxHashMapDenyLists, FxHashSet<u32>>(input)
}

pub fn p1_btreemap_fxhashset(input: Input) -> u32 { p1::<BTreeSetDenyLists, FxHashSet<u32>>(input) }

pub fn p1_fxhashmap_vec(input: Input) -> u32 { p1::<FxHashMapDenyLists, Vec<u32>>(input) }

pub fn p1_btreemap_vec(input: Input) -> u32 { p1::<BTreeSetDenyLists, Vec<u32>>(input) }
