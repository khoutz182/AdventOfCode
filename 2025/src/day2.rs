use std::ops::RangeInclusive;

use crate::utils::read_lines;

struct Products {
    ranges: Vec<RangeInclusive<u64>>,
}

impl From<&str> for Products {
    fn from(value: &str) -> Self {
        let ranges: Vec<RangeInclusive<u64>> = value
            .split(",")
            .map(|x| {
                let (start, end) = x.split_once("-").unwrap();
                start.parse().unwrap()..=end.parse().unwrap()
            })
            .collect();

        Self { ranges }
    }
}

impl Products {
    fn invalid_ids(&self) -> Vec<u64> {
        let mut products: Vec<u64> = vec![];
        for range in &self.ranges {
            for id in range.clone() {
                let product_id = id.to_string();
                let (first, second) = product_id.split_at(product_id.len() / 2);
                if first == second {
                    products.push(id);
                }
            }
        }

        products
    }

    fn repeats(input: &str, frequency: usize) -> bool {
        let (initial, mut remaining) = input.split_at(frequency);

        while !remaining.is_empty() {
            let (part, rem) = remaining.split_at(frequency);
            if part != initial {
                return false;
            }
            remaining = rem;
        }

        true
    }

    fn invalid_ids_pt2(&self) -> Vec<u64> {
        let mut products: Vec<u64> = vec![];
        for range in &self.ranges {
            for id in range.clone() {
                let product_id = id.to_string();
                for len in 1..=product_id.len() / 2 {
                    if product_id.len() % len != 0 {
                        continue;
                    }
                    if Self::repeats(&product_id, len) {
                        products.push(id);
                        break;
                    }
                }
            }
        }

        products
    }
}

pub fn run() {
    let line: String = read_lines("2")
        .unwrap()
        .map_while(Result::ok)
        .last()
        .unwrap();
    let products = Products::from(line.as_str());
    let invalid_ids = products.invalid_ids();
    let sum_of_ids: u64 = invalid_ids.iter().sum();
    println!("sum: {sum_of_ids}");

    let invalid_ids_pt2 = products.invalid_ids_pt2();
    let sum_of_ids_pt2: u64 = invalid_ids_pt2.iter().sum();
    println!("sum of pt2: {sum_of_ids_pt2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let products = Products::from(input);
        let invalid_ids = products.invalid_ids();
        let sum_of_ids: u64 = invalid_ids.iter().sum();

        assert_eq!(
            vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859],
            invalid_ids
        );
        assert_eq!(1227775554, sum_of_ids);

        assert!(Products::repeats("1188511885", 5));
        assert!(Products::repeats("11", 1));
        assert!(Products::repeats("22", 1));

        let pt2 = products.invalid_ids_pt2();
        assert_eq!(
            vec![
                11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656,
                824824824, 2121212121
            ],
            pt2
        );

        let input = "222220-222224";
        let products = Products::from(input);
        assert_eq!(products.invalid_ids_pt2(), vec![222222]);
    }
}
