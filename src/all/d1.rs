use std::{collections::HashMap, fmt, iter};

use bitvec::vec::BitVec;

use crate::Parse;

#[derive(Clone)]
pub struct Input {
    left:  Vec<u32>,
    right: Vec<u32>,
}

impl Parse for Input {
    fn parse(input: &str) -> Self {
        let (left, right): (Vec<_>, Vec<_>) = input
            .lines()
            .filter_map(|line| {
                line.split_once(' ').map(|(left, right)| {
                    (left.parse::<u32>().unwrap(), right.trim_ascii_start().parse::<u32>().unwrap())
                })
            })
            .unzip();
        Self { left, right }
    }
}

pub fn p1_zip(Input { mut left, mut right }: Input) -> impl fmt::Display {
    left.sort_unstable();
    right.sort_unstable();

    iter::zip(left, right).map(|(l, r)| l.max(r) - l.min(r)).sum::<u32>()
}

// Similar to `uniq -c`
struct UniqueIterator<I, T> {
    iter: I,
    peek: Option<(T, u32)>,
}

impl<I, T> Iterator for UniqueIterator<I, T>
where
    I: Iterator<Item = T>,
    T: Copy + Eq,
{
    type Item = (T, u32);

    fn next(&mut self) -> Option<Self::Item> {
        for next in self.iter.by_ref() {
            match &mut self.peek {
                &mut Some((prev, ref mut count)) if prev == next => {
                    *count += 1;
                }
                &mut Some((prev, count)) => {
                    self.peek = Some((next, 1));
                    return Some((prev, count));
                }
                None => {
                    self.peek = Some((next, 1));
                }
            }
        }

        self.peek.take()
    }
}

fn unique<T: Copy + Eq>(iter: impl IntoIterator<Item = T>) -> impl Iterator<Item = (T, u32)> {
    UniqueIterator { iter: iter.into_iter(), peek: None }
}

pub fn p2_hash(Input { left, right }: Input) -> impl fmt::Display {
    let mut counts = HashMap::<u32, u32>::new();
    for item in right {
        *counts.entry(item).or_default() += 1;
    }

    left.into_iter().map(|item| item * counts.get(&item).copied().unwrap_or_default()).sum::<u32>()
}

pub fn p2_sorted(Input { mut left, mut right }: Input) -> impl fmt::Display {
    left.sort_unstable();
    right.sort_unstable();

    let mut right = unique(right).peekable();

    let mut output = 0u32;

    for (item, left_count) in unique(left) {
        'right_loop: while let Some(&(right_item, right_count)) = right.peek() {
            if right_item < item {
                right.next().unwrap();
                continue;
            }

            if item == right_item {
                right.next().unwrap();
                output += item * left_count * right_count;
            }

            break 'right_loop;
        }
    }

    output
}

pub fn p2_count(Input { left, right }: Input) -> impl fmt::Display {
    fn collect_buckets(items: Vec<u32>) -> Vec<u32> {
        let mut output = vec![0u32; 100000];
        for item in items {
            output[item as usize] += 1;
        }
        output
    }

    let left = collect_buckets(left);
    let right = collect_buckets(right);

    iter::zip(left, right).enumerate().map(|(i, (l, r))| (i as u32) * l * r).sum::<u32>()
}

pub fn p2_bitvec(Input { left, right }: Input) -> impl fmt::Display {
    fn collect_buckets(items: Vec<u32>) -> (BitVec, Vec<u32>) {
        let mut presence: BitVec = iter::repeat(false).take(100000).collect();
        let mut output = vec![0u32; 100000];
        for item in items {
            presence.set(item as usize, true);
            output[item as usize] += 1;
        }
        (presence, output)
    }

    let (left_presence, left) = collect_buckets(left);
    let (right_presence, right) = collect_buckets(right);

    let presence = left_presence & right_presence;

    presence.iter_ones().map(|i| left[i] * right[i] * (i as u32)).sum::<u32>()
}
