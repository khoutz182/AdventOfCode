use crate::utils::read_lines;

struct Bank {
    batteries: Vec<u8>,
}
struct Power {
    banks: Vec<Bank>,
}

impl From<&str> for Bank {
    fn from(value: &str) -> Self {
        Self {
            batteries: value
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }
}
impl From<Vec<String>> for Power {
    fn from(value: Vec<String>) -> Self {
        let mut banks: Vec<Bank> = vec![];
        for line in value {
            banks.push(Bank::from(line.as_str()));
        }

        Self { banks }
    }
}

impl Bank {
    fn largest_joltage(&self) -> u8 {
        let first_range = &self.batteries[..self.batteries.len() - 1];
        let first = first_range.iter().max().unwrap();
        let first_idx = first_range
            .iter()
            .enumerate()
            .find(|(_, val)| *val == first)
            .unwrap()
            .0;
        let second = self.batteries[first_idx + 1..].iter().max().unwrap();

        first * 10 + second
    }

    fn jolt(batteries: &[u8], magnitude: usize) -> Vec<u8> {
        if magnitude == 1 {
            return vec![*batteries.iter().max().unwrap()];
        }
        let range = &batteries[..batteries.len() - (magnitude - 1)];
        let max = range.iter().max().unwrap();
        let idx = range
            .iter()
            .enumerate()
            .find(|(_, val)| *val == max)
            .unwrap()
            .0;

        let remaining_batteries = &batteries[idx + 1..];

        let mut ret = Self::jolt(remaining_batteries, magnitude - 1);
        ret.push(*max);

        ret
    }

    fn joltage(&self, magnitude: usize) -> u64 {
        Self::jolt(&self.batteries, magnitude)
            .iter()
            .rev()
            .fold(0, |acc, x| (acc * 10) + *x as u64)
    }
}

impl Power {
    fn sum_joltages(&self) -> u64 {
        let mut sum: u64 = 0;
        for bank in &self.banks {
            sum += bank.largest_joltage() as u64;
        }
        sum
    }

    fn boosted(&self, magnitude: usize) -> u64 {
        self.banks
            .iter()
            .fold(0, |acc, x| acc + x.joltage(magnitude))
    }
}

pub fn run() {
    let lines: Vec<String> = read_lines("3").unwrap().map_while(Result::ok).collect();
    let power = Power::from(lines);
    let sum = power.sum_joltages();
    println!("sum: {sum}");

    let boosted = power.boosted(12);
    println!("boosted power: {boosted}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(987654321111, Bank::from("987654321111111").joltage(12));
        assert_eq!(811111111119, Bank::from("811111111111119").joltage(12));
        assert_eq!(434234234278, Bank::from("234234234234278").joltage(12));
        assert_eq!(888911112111, Bank::from("818181911112111").joltage(12));
    }
}
