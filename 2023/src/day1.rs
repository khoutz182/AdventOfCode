use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    // let input = "./inputs/day1_sample.txt";
    let input = "./inputs/day1.txt";
    if let Ok(lines) = read_lines(input) {
        let mut total = 0u32;
        for line in lines {
            if let Ok(valid_line) = line {
                let line_number = (get_first_digit(&valid_line) * 10) + get_last_digit(&valid_line);
                total += line_number;
            }
        }
        println!("total: {}", total)
    } else {
        println!("{:?}", read_lines(input));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_first_digit(line: &String) -> u32 {
    let fixes = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut first_digit: u32 = 0;
    let mut first_digit_idx: usize = line.len();
    for fix in fixes {
        if let Some(index) = line.find(fix.0) {
            if index <= first_digit_idx {
                first_digit = fix.1;
                first_digit_idx = index;
            }
        }
    }
    if let Some(raw_index) = line.find(|ch: char| ch.is_digit(10)) {
        if raw_index <= first_digit_idx {
            first_digit = line.chars().nth(raw_index).unwrap().to_digit(10).unwrap();
        }
    }

    return first_digit;
}

fn get_last_digit(line: &String) -> u32 {
    let fixes = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut last_digit: u32 = 0;
    let mut last_digit_idx: usize = 0;
    for fix in fixes {
        if let Some(index) = line.rfind(fix.0) {
            if index >= last_digit_idx {
                last_digit = fix.1;
                last_digit_idx = index;
            }
        }
    }
    if let Some(raw_index) = line.rfind(|ch: char| ch.is_digit(10)) {
        if raw_index >= last_digit_idx {
            last_digit = line.chars().nth(raw_index).unwrap().to_digit(10).unwrap();
        }
    }

    return last_digit;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_digit(line: &str) -> u32 {
        return get_first_digit(&line.to_string());
    }

    fn last_digit(line: &str) -> u32 {
        return get_last_digit(&line.to_string());
    }

    #[test]
    fn test_first_and_last_digit() {
        let line = "2asksadflkj";
        assert_eq!(2, first_digit(line));
        assert_eq!(2, last_digit(line));
    }
}
