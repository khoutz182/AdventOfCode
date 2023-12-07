use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let input = "./inputs/day1_sample.txt";
    if let Ok(lines) = read_lines(input) {
        let mut total = 0u32;
        for line in lines {
            if let Ok(mangled) = line {
                let fixed_line = fix_line(&mangled);
                let mut line_digits: Vec<u32> = Vec::new();
                for char in fixed_line.chars() {
                    if let Some(digit) = char.to_digit(10) {
                        line_digits.push(digit);
                    }
                }
                let line_total = (line_digits[0] * 10) + line_digits[line_digits.len() - 1];
                println!("line total: {} for line '{}'", line_total, mangled);
                total += line_total;
            }
        }
        println!("total: {}", total)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn fix_line(broken_line: &String) -> String {
    let fixes = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut first_index: Option<(usize, (&str, &str))> = None;
    let mut last_index: Option<(usize, (&str, &str))> = None;
    for fix in fixes {
        if let Some(index) = broken_line.find(fix.0) {
            match first_index {
                None => first_index = Some((index, fix)),
                Some(index_with_fix) => {
                    if index < index_with_fix.0 {
                        first_index = Some((index, fix))
                    }
                }
            }
        }

        if let Some(index) = broken_line.rfind(fix.0) {
            match last_index {
                None => last_index = Some((index, fix)),
                Some(index_with_fix) => {
                    if index < index_with_fix.0 {
                        last_index = Some((index, fix))
                    }
                }
            }
        }
    }

    let mut line: String = broken_line.to_string();
    if let Some(f_index) = first_index {
        if let Some(index) = line.find(f_index.1.0) {
            let end_range = index + f_index.1.0.len();
            line.replace_range(index..end_range, f_index.1.1);
        }
    }
    if let Some(l_index) = last_index {
        if let Some(index) = line.find(l_index.1.0) {
            let end_range = index + l_index.1.0.len();
            line.replace_range(index..end_range, l_index.1.1);
        }
    }

    return line;
}

fn fix_line_bad(broken_line: &String) -> String {
    let mut line: String = broken_line.to_string();
    while let Some(fix) = find_first_fix(&line) {
        line = line.replace(&fix.0, &fix.1).to_string();
    }
    return line;
}

fn find_first_fix(line: &String) -> Option<(&str, &str)> {
    let fixes = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut current_index: Option<(usize, (&str, &str))> = None;
    for fix in fixes {
        if let Some(index) = line.find(fix.0) {
            match current_index {
                None => current_index = Some((index, fix)),
                Some(index_with_fix) => {
                    if index < index_with_fix.0 {
                        current_index = Some((index, fix))
                    }
                }
            }
        }
    }

    if let Some(index) = current_index {
        return Some(index.1);
    } else {
        return None;
    }
}
