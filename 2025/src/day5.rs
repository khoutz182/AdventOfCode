use std::{fmt::Display, ops::RangeInclusive};

use crate::utils::read_lines;

struct Inventory {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

impl From<&Vec<&str>> for Inventory {
    fn from(value: &Vec<&str>) -> Self {
        let fresh_ranges: Vec<RangeInclusive<u64>> = value
            .iter()
            .take_while(|l| !(**l).is_empty()) // hate this syntax
            .map(|l| {
                let (lower, upper) = (*l).split_once("-").unwrap();
                lower.parse().unwrap()..=upper.parse().unwrap()
            })
            .collect();

        let ids: Vec<u64> = value
            .iter()
            .skip_while(|l| !(**l).is_empty())
            .skip(1)
            .map(|l| (*l).parse().unwrap())
            .collect();

        Self { fresh_ranges, ids }
    }
}

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display: String = self
            .fresh_ranges
            .iter()
            .enumerate()
            .flat_map(|(i, r)| {
                vec![
                    i.to_string(),
                    ": ".to_string(),
                    r.start().to_string(),
                    "-".to_string(),
                    r.end().to_string(),
                    "\n".to_string(),
                ]
            })
            .collect();

        write!(f, "{display}")
    }
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return ranges;
    }
    ranges.sort_by_key(|range| *range.start());

    let mut merged = Vec::new();
    let mut start = ranges[0].start();
    let mut end = ranges[0].end();
    for range in ranges.iter().skip(1) {
        if range.start() > end {
            merged.push(RangeInclusive::new(*start, *end));
            start = range.start();
            end = range.end();
        }
        if range.end() > end {
            end = range.end();
        }
    }
    merged.push(RangeInclusive::new(*start, *end));

    merged
}

impl Inventory {
    fn has_overlaps(&self) -> bool {
        self.fresh_ranges
            .iter()
            .zip(self.fresh_ranges.iter().skip(1))
            .any(|(a, b)| a.end() >= b.start())
    }

    fn merge_ranges(&mut self) {
        self.fresh_ranges.sort_by(|a, b| a.start().cmp(b.start()));
        self.ids.sort();

        let mut prev = self.fresh_ranges.first().unwrap().clone();
        let mut merged_ranges = vec![];

        for range in self.fresh_ranges.iter().skip(1) {
            if *prev.end() >= *range.start() {
                prev = *prev.start()..=*range.end();
            } else {
                merged_ranges.push(prev.clone());
                prev = range.clone();
            }
        }

        merged_ranges.push(prev);
        self.fresh_ranges = merged_ranges;
    }

    fn count_fresh(&self) -> usize {
        let mut count = 0;
        for id in &self.ids {
            if self.fresh_ranges.iter().any(|r| r.contains(id)) {
                count += 1;
            }
        }

        count
    }

    fn total_fresh(&self, start_limit: u64) -> u64 {
        self.fresh_ranges
            .iter()
            .filter(|r| *r.start() <= start_limit)
            .map(|r| (r.end() - r.start()) + 1)
            .peekable()
            .sum()
    }
}

pub fn run() {
    let lines: Vec<String> = read_lines("5").unwrap().map_while(Result::ok).collect();
    let partially_owned = lines.iter().map(String::as_str).collect::<Vec<&str>>();

    let mut valid_inventory = Inventory::from(&partially_owned);
    let mut my_inventory = Inventory::from(&partially_owned);

    valid_inventory
        .fresh_ranges
        .sort_by(|a, b| a.start().cmp(b.start()));
    valid_inventory.fresh_ranges = merge_ranges(valid_inventory.fresh_ranges);
    my_inventory.merge_ranges();

    for limiting_range in my_inventory.fresh_ranges.iter().skip(2) {
        let limit = *limiting_range.start();
        let my_total_fresh = my_inventory.total_fresh(limit);
        let their_total_fresh = valid_inventory.total_fresh(limit);

        println!(
            "limit of: {limit}, mine: {my_total_fresh}, theirs: {their_total_fresh}\tdiff: {}",
            my_total_fresh.abs_diff(their_total_fresh)
        );
    }

    // let my_solution: u64 = 321713401631775;
    // println!("my solution was {my_solution}");
    // println!("total fresh:    {total_fresh}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = vec![
            "3-5", "10-14", "16-20", "12-18", "11-13", "", "1", "5", "8", "11", "17", "32",
        ];
        let mut inventory = Inventory::from(&sample);
        inventory.merge_ranges();
        println!("inventory post-merged:  {inventory}");
        let fresh = inventory.count_fresh();
        assert_eq!(3, fresh);

        assert!(!inventory.has_overlaps());
        assert_eq!(14, inventory.total_fresh(u64::MAX));
    }
}
