type Sum = u32;

pub fn p1_find(input: String) -> Sum {
    let mut input = input.as_str();

    let mut sum = 0;
    while let Some(index) = input.find("mul(") {
        input = &input[index + 4..];
        if let Some(left) = parse_int(&mut input) {
            if strip_prefix_char_mut(&mut input, ',') {
                if let Some(right) = parse_int(&mut input) {
                    if strip_prefix_char_mut(&mut input, ')') {
                        sum += left * right;
                    }
                }
            }
        }
    }
    sum
}

pub fn p2_find(input: String) -> Sum {
    enum Mode {
        Do,
        Dont,
    }

    impl Mode {
        fn find_pattern(&self) -> &'static [char] {
            match self {
                Mode::Do => &['m', 'd'],
                Mode::Dont => &['d'],
            }
        }
    }

    let mut input = input.as_str();
    let mut mode = Mode::Do;
    let mut sum = 0;
    while let Some(index) = input.find(mode.find_pattern()) {
        input = &input[index..];
        match mode {
            Mode::Dont => {
                if strip_prefix_str_mut(&mut input, "do()") {
                    mode = Mode::Do;
                } else {
                    // skip current index
                    input = &input[1..];
                }
            }
            Mode::Do => {
                if strip_prefix_str_mut(&mut input, "don't()") {
                    mode = Mode::Dont;
                } else if strip_prefix_str_mut(&mut input, "mul(") {
                    if let Some(left) = parse_int(&mut input) {
                        if let Some(stripped) = input.strip_prefix(',') {
                            input = stripped;
                            if let Some(right) = parse_int(&mut input) {
                                if let Some(stripped) = input.strip_prefix(')') {
                                    input = stripped;
                                    sum += left * right;
                                }
                            }
                        }
                    }
                } else {
                    // skip current index
                    input = &input[1..];
                }
            }
        }
    }
    sum
}

fn strip_prefix_char_mut(input: &mut &str, prefix: char) -> bool {
    if let Some(stripped) = input.strip_prefix(prefix) {
        *input = stripped;
        true
    } else {
        false
    }
}

fn strip_prefix_str_mut(input: &mut &str, prefix: &str) -> bool {
    if let Some(stripped) = input.strip_prefix(prefix) {
        *input = stripped;
        true
    } else {
        false
    }
}

fn parse_int(input: &mut &str) -> Option<Sum> {
    let bytes = input.as_bytes();
    if bytes.first().is_some_and(u8::is_ascii_digit) {
        let value = Sum::from(bytes[0] - b'0');
        if bytes.get(1).is_some_and(u8::is_ascii_digit) {
            let value = value * 10 + Sum::from(bytes[1] - b'0');
            if bytes.get(2).is_some_and(u8::is_ascii_digit) {
                let value = value * 10 + Sum::from(bytes[2] - b'0');
                *input = &input[3..];
                return Some(value);
            }
            *input = &input[2..];
            return Some(value);
        }
        *input = &input[1..];
        return Some(value);
    }
    None
}
