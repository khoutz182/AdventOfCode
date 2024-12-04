use crate::utils::read_lines;

pub fn run() {}

fn add_instructions(line: &str) -> i32 {
    let scanner = Scanner::new(line);

    3
}

struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let sum = add_instructions(input);

        assert_eq!(161, sum, "Can't even do the sample? what a scrub");
    }
}
