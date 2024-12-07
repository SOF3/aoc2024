use std::collections::BTreeSet;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::Parse;

#[derive(Clone)]
pub struct Input {
    constraints: Vec<Constraint>,
    updates: Vec<Update>,
}

#[derive(Clone)]
struct Constraint {
    earlier: u32,
    later: u32,
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
            constraints.push(Constraint { earlier: left.parse().unwrap(), later: right.parse().unwrap() });
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

pub fn p1_fxhash(input: Input) -> u32 {
    let mut deny_lists: FxHashMap<u32, Vec<u32>> = FxHashMap::default();

    for constraint in input.constraints {
        deny_lists.entry(constraint.later).or_default().push(constraint.earlier);
    }

    let mut result = 0;

    'update: for Update(items) in input.updates {
        let mut disallowed: FxHashSet<u32> = FxHashSet::default();

        for &item in &items {
            if disallowed.contains(&item) {
                continue 'update;
            }

            if let Some(deny_list) = deny_lists.get(&item) {
                disallowed.extend(deny_list);
            }
        }

        result+=items[items.len()/2];
    }

    result
}

pub fn p1_btreemap_fxhashset(input: Input) -> u32 {
    let mut deny_lists: BTreeSet<(u32, u32)> = BTreeSet::default();

    for constraint in input.constraints {
        deny_lists.insert((constraint.later, constraint.earlier));
    }

    let mut result = 0;

    'update: for Update(items) in input.updates {
        let mut disallowed: FxHashSet<u32> = FxHashSet::default();

        for &item in &items {
            if disallowed.contains(&item) {
                continue 'update;
            }

            disallowed.extend(deny_lists.range((item, 0)..=(item, u32::MAX)).map(|&(_, dep)| dep));
        }

        result+=items[items.len()/2];
    }

    result
}
