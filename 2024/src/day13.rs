use std::num::TryFromIntError;

use crate::utils::read_lines;

#[derive(Debug)]
struct Machine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

impl Machine {
    fn is_solution(&self, a: usize, b: usize) -> bool {
        let location = (
            self.button_a.0 * a + self.button_b.0 * b,
            self.button_a.1 * a + self.button_b.1 * b,
        );

        location == self.prize
    }

    fn solve(&self) -> Option<usize> {
        let mut min_cost: Option<usize> = None;
        (1..=100).for_each(|i| {
            (1..=100).for_each(|j| {
                if self.is_solution(i, j) {
                    let cost = i * 3 + j;
                    if let Some(c) = min_cost {
                        if cost < c {
                            min_cost = Some(cost);
                        }
                    } else {
                        min_cost = Some(cost);
                    }
                }
            });
        });

        min_cost
    }

    fn unique(&self) -> bool {
        self.button_a.0 * self.button_a.1 != self.button_b.0 * self.button_b.1
    }

    // a1*b2 - a2*b1
    fn cross_mult_1(&self) -> i128 {
        (self.button_a.0 * self.button_b.1) as i128 - (self.button_a.1 * self.button_b.0) as i128
    }

    // b1*c2 - b2*c1
    fn cross_mult_2(&self) -> i128 {
        (self.button_a.1 * self.prize.1) as i128 - (self.button_b.1 * self.prize.0) as i128
    }

    // a1*c2 - a2*c1
    fn cross_mult_3(&self) -> i128 {
        (self.button_a.0 * self.prize.1) as i128 - (self.button_b.0 * self.prize.1) as i128
    }

    fn inconsistent(&self) -> bool {
        if self.cross_mult_1() != 0 {
            return false;
        }

        if self.cross_mult_2() == 0 {
            return false;
        }

        if self.cross_mult_3() == 0 {
            return false;
        }

        true
    }

    fn infinite_solutions(&self) -> bool {
        if self.cross_mult_1() != 0 {
            return false;
        }

        if self.cross_mult_2() != 0 {
            return false;
        }

        if self.cross_mult_3() != 0 {
            return false;
        }

        true
    }

    fn solve_d2(&self) -> Option<usize> {
        if !self.unique() || self.inconsistent() || self.infinite_solutions() {
            println!(
                "self: {:?}, unique: {}, inconsistent: {}, infinite: {}",
                self,
                self.unique(),
                self.inconsistent(),
                self.infinite_solutions()
            );
            return None;
        }

        let denominator = (self.button_b.1 as i128 * self.button_a.0 as i128)
            - (self.button_b.0 as i128 * self.button_a.1 as i128);

        if denominator == 0 {
            return None;
        }

        let a_numerator: i128 = (-(self.button_b.0 as i128) * self.prize.1 as i128)
            - (-(self.prize.0 as i128) * self.button_b.1 as i128);
        let b_numerator: i128 = (-(self.prize.0 as i128) * self.button_a.1 as i128)
            - (-(self.prize.1 as i128) * self.button_a.0 as i128);

        if a_numerator % denominator != 0 || b_numerator % denominator != 0 {
            return None;
        }
        let a_presses: i128 = a_numerator / denominator;
        let b_presses: i128 = b_numerator / denominator;

        let cost: Result<usize, TryFromIntError> = (a_presses * 3 + b_presses).try_into();

        cost.ok()
    }
}

#[derive(Debug)]
struct Arcade {
    machines: Vec<Machine>,
}

impl Arcade {
    fn solve(&self) -> usize {
        self.machines
            .iter()
            .filter_map(|machine| machine.solve())
            .sum()
    }

    fn solve_d2(&self) -> usize {
        self.machines
            .iter()
            .filter_map(|machine| machine.solve_d2())
            .sum()
    }
}

fn extract_pair(line: String, num_prefix: &str) -> (usize, usize) {
    let numbers: Vec<usize> = line
        .split_once(":")
        .unwrap()
        .1
        .split(",")
        .map(|x| x.trim())
        .map(|x| x.split_once(num_prefix).unwrap().1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    match &numbers[..] {
        &[first, second, ..] => (first, second),
        _ => unreachable!(),
    }
}

impl From<Vec<String>> for Arcade {
    fn from(lines: Vec<String>) -> Self {
        let mut machines: Vec<Machine> = vec![];

        let mut line_iter = lines.into_iter();
        loop {
            let b_a_line = line_iter.next();
            if b_a_line.is_none() {
                break;
            }
            let button_a = extract_pair(b_a_line.unwrap(), "+");
            let button_b = extract_pair(line_iter.next().unwrap(), "+");
            let prize = extract_pair(line_iter.next().unwrap(), "=");
            let d2_prize = (prize.0 + 10000000000000, prize.1 + 10000000000000);

            machines.push(Machine {
                button_a,
                button_b,
                prize: d2_prize,
            });

            if line_iter.next().is_none() {
                break;
            }
        }
        Arcade { machines }
    }
}

pub fn run() {
    let arcade: Arcade = if let Ok(lines) = read_lines("13") {
        let all_lines: Vec<String> = lines.map_while(Result::ok).collect();
        Arcade::from(all_lines)
    } else {
        panic!("ahhhhh");
    };

    // let solution = arcade.solve();
    let solution = arcade.solve_d2();
    println!("solution: {solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13() {
        assert_eq!(
            (94, 34),
            extract_pair("Button A: X+94, Y+34".to_string(), "+")
        );
        assert_eq!(
            (8400, 5400),
            extract_pair("Prize: X=8400, Y=5400".to_string(), "=")
        );
        let lines: Vec<String> = vec![
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
            "",
            "Button A: X+26, Y+66",
            "Button B: X+67, Y+21",
            "Prize: X=12748, Y=12176",
            "",
            "Button A: X+17, Y+86",
            "Button B: X+84, Y+37",
            "Prize: X=7870, Y=6450",
            "",
            "Button A: X+69, Y+23",
            "Button B: X+27, Y+71",
            "Prize: X=18641, Y=10279",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        let arcade = Arcade::from(lines);
        // assert_eq!(480, arcade.solve());
        assert_eq!(1545093008499, arcade.solve_d2());
    }
}
