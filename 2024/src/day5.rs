use std::{cmp::Ordering, str::FromStr};

use crate::utils::read_lines;

#[derive(Debug)]
struct RulesAndUpdates {
    rules: Vec<OrderRule>,
    updates: Vec<PageUpdates>,
}

type PageUpdates = Vec<u16>;
type OrderRule = (u16, u16);

impl RulesAndUpdates {
    fn new() -> Self {
        RulesAndUpdates {
            rules: vec![],
            updates: vec![],
        }
    }
}

pub fn run() {
    let input = parse_input(|| read_lines("5"));
    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("day 5 result:");
    println!("part 1 sum: {}", part1);
    println!("part 2 sum: {}", part2);
}

fn part_1(input: &RulesAndUpdates) -> u16 {
    input
        .updates
        .iter()
        .filter(|update| is_valid(update, &input.rules))
        .map(get_mid)
        .sum()
}

fn part_2(input: &RulesAndUpdates) -> u16 {
    input
        .updates
        .clone()
        .iter_mut()
        .filter(|update| !is_valid(update, &input.rules))
        .map(|update| {
            fix_update(update, &input.rules);
            get_mid(update)
        })
        .sum()
}

fn is_valid(page_update: &PageUpdates, rules: &[OrderRule]) -> bool {
    for left in 0..page_update.len() - 1 {
        for right in left + 1..page_update.len() {
            let violations = rules
                .iter()
                .any(|rule| rule.0 == page_update[right] && rule.1 == page_update[left]);
            if violations {
                return false;
            }
        }
    }

    true
}

fn fix_update(page_update: &mut PageUpdates, rules: &[OrderRule]) {
    page_update.sort_by(|left, right| {
        for rule in rules {
            if rule.0 == *left && rule.1 == *right {
                return Ordering::Less;
            }
            if rule.0 == *right && rule.1 == *left {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    });
}

fn get_mid(page_update: &PageUpdates) -> u16 {
    page_update[page_update.len() / 2]
}

fn parse_input<F, E>(f: F) -> RulesAndUpdates
where
    F: FnOnce() -> Result<std::io::Lines<std::io::BufReader<std::fs::File>>, E>,
{
    let lines_result = f();
    let mut input = RulesAndUpdates::new();
    let mut rules = true;
    if let Ok(lines) = lines_result {
        for line in lines.map_while(Result::ok) {
            if line.trim().is_empty() {
                rules = false;
                continue;
            }

            if rules {
                if let Some((left, right)) = line.split_once('|') {
                    input
                        .rules
                        .push((u16::from_str(left).unwrap(), u16::from_str(right).unwrap()))
                }
            } else {
                input
                    .updates
                    .push(line.split(',').map(|s| s.parse().unwrap()).collect());
            }
        }
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = parse_input(|| read_lines("5_sample"));
        let part1 = part_1(&input);
        let part2 = part_2(&input);

        assert_eq!(143, part1);
        assert_eq!(123, part2);
    }
}
