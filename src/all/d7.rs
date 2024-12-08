use std::iter;

fn fast_parse_once(input: &[u8], delim: u8) -> Option<(&[u8], u64)> {
    let mut buf = input;
    let mut output = 0;
    while buf.first().is_some_and(|&digit| digit != delim) {
        output *= 10;
        let (&first, rest) = buf.split_first().unwrap();
        output += u64::from(first - b'0');
        buf = rest;
    }

    if buf.len() != input.len() {
        Some((buf, output))
    } else {
        None
    }
}

fn fast_parse_once_reverse(input: &[u8], delim: u8) -> Option<(&[u8], Operand)> {
    let mut buf = input;
    let mut output = 0;
    let mut unit = 1;
    while buf.last().is_some_and(|&digit| digit != delim) {
        let (&last, rest) = buf.split_last().unwrap();
        output += u64::from(last - b'0') * unit;
        unit *= 10;
        buf = rest;
    }

    if buf.len() != input.len() {
        Some((buf, Operand { value: output, bytes: &input[buf.len()..] }))
    } else {
        None
    }
}

#[derive(Clone, Copy)]
struct Operand<'a> {
    value: u64,
    bytes: &'a [u8],
}

fn parse(input: &str) -> impl Iterator<Item = Line> {
    input.lines().map(|line| {
        let (operands, result) = fast_parse_once(line.as_bytes(), b':').unwrap();
        Line { result, operands }
    })
}

impl<'a> Line<'a> {
    fn operands_rev(&self) -> impl Iterator<Item = Operand> + Clone {
        let mut operands = self.operands;
        iter::from_fn(move || {
            operands = operands.trim_ascii_end();
            if operands.last() == Some(&b':') {
                return None;
            }
            let (rest, last) = fast_parse_once_reverse(operands, b' ')?;
            operands = rest;
            Some(last)
        })
    }
}

struct Line<'a> {
    result:   u64,
    operands: &'a [u8],
}

// Try iterating from the back.
// Break early if unable to reverse addition (negative) or multiplication (not divisible).
fn is_valid_reverse_recurse_p1<'a>(
    result: u64,
    mut operands: impl Iterator<Item = Operand<'a>> + Clone,
) -> bool {
    match operands.next() {
        None => result == 0, // empty list
        Some(Operand { value: last, .. }) => {
            // Try addition
            if let Some(intermediate) = result.checked_sub(last) {
                if is_valid_reverse_recurse_p1(intermediate, operands.clone()) {
                    return true;
                }
            }

            // Try multiplication
            if result % last == 0 {
                // operands are always nonzero
                if is_valid_reverse_recurse_p1(result / last, operands) {
                    return true;
                }
            }

            false
        }
    }
}

pub fn p1_reversed(input: String) -> u64 {
    parse(&input)
        .filter(|line| is_valid_reverse_recurse_p1(line.result, line.operands_rev()))
        .map(|line| line.result)
        .sum()
}

// Try iterating from the back.
// Break early if unable to reverse addition (negative),
// concatenation (not divisible after subtraction) or multiplication (not divisible).
fn is_valid_reverse_recurse_p2<'a>(
    result: u64,
    mut operands: impl Iterator<Item = Operand<'a>> + Clone,
) -> bool {
    match operands.next() {
        None => result == 0, // empty list
        Some(last) => {
            // Try addition
            if let Some(intermediate) = result.checked_sub(last.value) {
                if is_valid_reverse_recurse_p2(intermediate, operands.clone()) {
                    return true;
                }
            }

            // Try concatenation
            if let Some(intermediate) = strip_base10_suffix(result, last) {
                if is_valid_reverse_recurse_p2(intermediate, operands.clone()) {
                    return true;
                }
            }

            // Try multiplication
            if result % last.value == 0 {
                // no operands equal to 0
                if is_valid_reverse_recurse_p2(result / last.value, operands) {
                    return true;
                }
            }

            false
        }
    }
}

fn strip_base10_suffix<'a>(long: u64, suffix: Operand<'a>) -> Option<u64> {
    let remain = long.wrapping_sub(suffix.value); // works on "my input"
    let unit = 10u64.pow(suffix.bytes.len() as u32);
    if remain % unit == 0 {
        Some(remain / unit)
    } else {
        None
    }
}

pub fn p2_reversed(input: String) -> u64 {
    parse(&input)
        .filter(|line| is_valid_reverse_recurse_p2(line.result, line.operands_rev()))
        .map(|line| line.result)
        .sum()
}
