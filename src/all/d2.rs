use std::fmt;

use itertools::Itertools;

use crate::Parse;

#[derive(Clone)]
pub struct Input(Vec<Line>);

#[derive(Clone)]
struct Line(Vec<u32>);

impl Parse for Input {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| Line(line.split(' ').map(|s| s.parse::<u32>().unwrap()).collect()))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Increase,
    Decrease,
    Illegal,
}

fn compare(left: u32, right: u32) -> Direction {
    if (right + 1..=right + 3).contains(&left) {
        Direction::Decrease
    } else if (left + 1..=left + 3).contains(&right) {
        Direction::Increase
    } else {
        Direction::Illegal
    }
}

fn is_safe_windows(levels: impl Iterator<Item = u32>) -> bool {
    let directions = levels.tuple_windows().map(|(left, right)| compare(left, right));
    directions.tuple_windows().all(|(left, right)| left != Direction::Illegal && left == right)
}

fn is_safe_first_all(levels: impl Iterator<Item = u32>) -> bool {
    let mut directions = levels.tuple_windows().map(|(left, right)| compare(left, right));
    let first = directions.next().unwrap();
    first != Direction::Illegal && directions.all(|d| d == first)
}

pub fn p1_windows(input: Input) -> impl fmt::Display {
    fn is_safe(line: &Line) -> bool { is_safe_windows(line.0.iter().copied()) }

    input.0.iter().filter(|line| is_safe(line)).count()
}

pub fn p1_first_all(input: Input) -> impl fmt::Display {
    fn is_safe(line: &Line) -> bool { is_safe_first_all(line.0.iter().copied()) }

    input.0.iter().filter(|line| is_safe(line)).count()
}

pub fn p2_brute_force(input: Input) -> impl fmt::Display {
    fn is_safe(line: &Line) -> bool {
        is_safe_first_all(line.0.iter().copied())
            || (0..line.0.len()).any(|skip| {
                is_safe_first_all(
                    line.0[..skip].iter().copied().chain(line.0[skip + 1..].iter().copied()),
                )
            })
    }

    input.0.iter().filter(|line| is_safe(line)).count()
}

// TODO: this answer is wrong
pub fn p2_vec(input: Input) -> impl fmt::Display {
    fn is_safe_skip(line: &Line, index: usize, dominant: Direction) -> bool {
        if let (Some(prev_index), Some(&next_level)) = (index.checked_sub(1), line.0.get(index + 1))
        {
            if compare(line.0[prev_index], next_level) != dominant {
                return false;
            }
        }

        true
    }

    fn is_safe(line: &Line) -> bool {
        let directions: Vec<_> =
            line.0.iter().tuple_windows().map(|(&left, &right)| compare(left, right)).collect();

        let increase_count = directions.iter().filter(|&&d| d == Direction::Increase).count();
        let dominants = if increase_count * 2 > directions.len() {
            &[Direction::Increase][..]
        } else if increase_count * 2 == directions.len() {
            &[Direction::Increase, Direction::Decrease][..]
        } else {
            &[Direction::Decrease][..]
        };

        dominants.iter().any(|&dominant| {
            let mut violations = directions.iter().enumerate().filter(|(_, &d)| d != dominant);
            if let Some((index, _)) = violations.next() {
                // line[index] -> line[index+1] violation,
                // attempt to skip either line[index] or line[index+1]

                if is_safe_skip(line, index, dominant) && is_safe_skip(line, index + 1, dominant) {
                    let mut next_violation = violations.next();
                    if next_violation.is_some_and(|(next_index, _)| next_index == index + 1) {
                        next_violation = violations.next();
                    }

                    next_violation.is_none()
                } else {
                    false
                }
            } else {
                true
            }
        })
    }

    input.0.iter().filter(|line| is_safe(line)).count()
}
