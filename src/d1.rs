use std::{fmt, iter};

struct Input {
    left:  Vec<u32>,
    right: Vec<u32>,
}

impl Input {
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

pub fn p1(input: &str) -> impl fmt::Display {
    let Input { mut left, mut right } = Input::parse(input);

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

pub fn p2_sorted(input: &str) -> impl fmt::Display {
    let Input { mut left, mut right } = Input::parse(input);

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
