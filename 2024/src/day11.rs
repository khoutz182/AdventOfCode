use std::{collections::HashMap, fmt::Display, vec::Vec};

struct Arrangement {
    stones: Vec<u64>,
}

impl From<&str> for Arrangement {
    fn from(value: &str) -> Self {
        Arrangement {
            stones: value
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }
}

impl Display for Arrangement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display: String = self
            .stones
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{display}")
    }
}

fn rec_blink(blink_count: usize, stone: u64, cache: &mut HashMap<(usize, u64), u64>) -> u64 {
    let key = (blink_count, stone);
    if cache.contains_key(&key) {
        return cache[&key];
    }
    if blink_count == 0 {
        return 1;
    }

    let count = if stone == 0 {
        rec_blink(blink_count - 1, 1, cache)
    } else if stone.to_string().len() % 2 == 0 {
        let num = stone.to_string();
        let (first, second) = num.split_at(num.len() / 2);
        rec_blink(blink_count - 1, first.parse().unwrap(), cache)
            + rec_blink(blink_count - 1, second.parse().unwrap(), cache)
    } else {
        rec_blink(blink_count - 1, stone * 2024, cache)
    };

    cache.insert(key, count);

    count
}

impl Arrangement {
    fn blink(self) -> Self {
        let mut new_stones: Vec<u64> = Vec::new();
        self.stones.iter().for_each(|s| {
            if *s == 0 {
                new_stones.push(1);
            } else if s.to_string().len() % 2 == 0 {
                let num = s.to_string();
                let (first, second) = num.split_at(num.len() / 2);
                new_stones.push(first.parse().unwrap());
                new_stones.push(second.parse().unwrap());
            } else {
                new_stones.push(s * 2024);
            }
        });

        Arrangement { stones: new_stones }
    }

    fn blink_and_count(self, blink_count: usize) -> u64 {
        let mut stone_count: u64 = 0;
        let mut cache: HashMap<(usize, u64), u64> = HashMap::new();
        self.stones.iter().for_each(|stone| {
            stone_count += rec_blink(blink_count, *stone, &mut cache);
        });

        stone_count
    }
}

pub fn run() {
    let input = "4 4841539 66 5279 49207 134 609568 0";
    let blinked = (0..25).fold(Arrangement::from(input), |acc, _| acc.blink());
    let stone_count = blinked.stones.len();
    println!("stones: {stone_count}");

    let arrangement = Arrangement::from(input);
    let giga_count = arrangement.blink_and_count(75);
    // let giga_blinked = (0..75).fold(Arrangement::from(input), |acc, _| acc.blink());
    // let giga_count = giga_blinked.stones.len();
    println!("giga stones: {giga_count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "125 17";

        let stones = Arrangement::from(input);
        assert_eq!("125 17", format!("{stones}"));
        let stone_count_a = stones.blink_and_count(25);
        assert_eq!(stone_count_a, 55312);

        // let blinked = (0..75).fold(Arrangement::from(input), |acc, _| acc.blink());
        // let stone_count = blinked.stones.len();
        // assert_eq!(55312, stone_count);
    }
}
