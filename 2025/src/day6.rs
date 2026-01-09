use std::ops::Range;

use crate::utils::read_lines;

enum Operation {
    Multiply,
    Addition,
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '+' => Operation::Addition,
            '*' => Operation::Multiply,
            _ => panic!("at the disco"),
        }
    }
}

fn parse_lines(lines: &Vec<String>) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let mut numbers: Vec<Vec<u64>> = vec![];
    let mut operations: Vec<Operation> = vec![];

    for line in lines {
        if line.chars().any(|c| c.is_ascii_digit()) {
            for (index, number) in line.split_whitespace().enumerate() {
                let num = number.parse::<u64>().unwrap();
                let existing_numbers = numbers.get_mut(index);
                if let Some(nmbrs) = existing_numbers {
                    nmbrs.push(num);
                } else {
                    numbers.push(vec![num]);
                }
            }

            continue;
        }
        operations = line
            .split_whitespace()
            .map(|op| match op {
                "+" => Operation::Addition,
                "*" => Operation::Multiply,
                _ => panic!("at the disco"),
            })
            .collect();

        break;
    }

    (numbers, operations)
}

fn parse_lines_pt2(lines: &[String]) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let last_line = lines.iter().last().unwrap();
    let (indexes, operations): (Vec<usize>, Vec<Operation>) = last_line
        .char_indices()
        .filter(|(_, char)| !char.is_whitespace())
        .map(|(idx, char)| (idx, Operation::from(char)))
        .unzip();
    let mut ranges: Vec<Range<usize>> = vec![];
    let mut first: usize = 0;
    indexes.iter().skip(1).for_each(|last| {
        ranges.push(first..(*last - 1));
        first = *last;
    });
    ranges.push(first..last_line.len());

    let mut slices: Vec<Vec<&str>> = vec![];

    for line in lines
        .iter()
        .take_while(|l| l.chars().all(|x| x.is_whitespace() || x.is_numeric()))
    {
        let str = line.as_str();
        for (idx, range) in ranges.iter().enumerate() {
            let slice = &str[range.clone()];
            if let Some(existing) = slices.get_mut(idx) {
                existing.push(slice);
            } else {
                slices.push(vec![slice]);
            }
        }
    }

    let numbers: Vec<Vec<u64>> = slices.iter().map(|c| transform(c.to_vec())).collect();

    (numbers, operations)
}

fn transform(chonk: Vec<&str>) -> Vec<u64> {
    let width = chonk.first().unwrap().len();
    let mut chars: Vec<Vec<char>> = vec![vec![]; width];
    for line in &chonk {
        for (idx, char) in line.char_indices() {
            chars.get_mut(idx).unwrap().push(char);
        }
    }

    chars
        .iter()
        .map(|c| c.iter().collect::<String>())
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect()
}

fn perform_operations(numbers: Vec<Vec<u64>>, operations: Vec<Operation>) -> Vec<u64> {
    numbers
        .iter()
        .zip(operations.iter())
        .map(|(numbers, op)| match op {
            Operation::Multiply => numbers.iter().product(),
            Operation::Addition => numbers.iter().sum(),
        })
        .collect()
}

pub fn run() {
    let lines = read_lines("6").unwrap().map_while(Result::ok).collect();
    let (numbers, ops) = parse_lines(&lines);
    let results = perform_operations(numbers, ops);
    println!("total: {}", results.iter().sum::<u64>());

    let (numbers, ops) = parse_lines_pt2(&lines);
    // println!("pt 2 numbers: {:?}", numbers);
    let results = perform_operations(numbers, ops);
    println!("total pt 2: {}", results.iter().sum::<u64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let (numbers, operations) = parse_lines(&input);
        assert_eq!(numbers.len(), operations.len());
        let results = perform_operations(numbers, operations);
        assert_eq!(vec![33210, 490, 4243455, 401], results);
        assert_eq!(4277556_u64, results.iter().sum());

        let (numbers, operations) = parse_lines_pt2(&input);
        let results = perform_operations(numbers, operations);
        assert_eq!(vec![8544, 625, 3253600, 1058], results);
        assert_eq!(3263827_u64, results.iter().sum());
    }
}
