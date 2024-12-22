use std::io::Result;

use crate::utils::read_lines;

#[derive(Debug)]
struct Equation {
    result: u64,
    values: Vec<u64>,
}

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq)]
enum Operation {
    Add,
    Multply,
    Concat,
}

impl Operation {
    fn run(&self, left: u64, right: u64) -> u64 {
        match self {
            Self::Add => left + right,
            Self::Multply => left * right,
            Self::Concat => {
                let mut pow = 10;
                while right >= pow {
                    pow *= 10;
                }
                left * pow + right
            }
        }
    }
}

impl Equation {
    fn is_valid(&self, ops: &[Operation]) -> bool {
        let combos = self.values.len() - 1;
        let combinations = get_combos(combos, ops);
        // let mut combinations = get_combos(combos, ops);

        // println!("result: {}, values: {:?}", self.result, self.values);
        // println!("combinations: {:?}", combinations);

        combinations.iter().any(|operations| {
            // combinations.any(|operations| {
            let mut val_iter = self.values.iter();
            let initial = val_iter.next().unwrap().to_owned();

            let operation_result = val_iter
                .zip(operations.clone())
                // .fold_while(initial, |acc, (x, op)| {
                //     if acc > self.result {
                //         Done(acc)
                //     } else {
                //         Continue(op.run(acc, *x))
                //     }
                // })
                // .into_inner();
                .fold(initial, |acc, (x, op)| op.run(acc, *x));

            // println!(
            //     "values: {:?}, operations: {:?}, result: {}, expected result: {}",
            //     self.values, operations, operation_result, self.result
            // );
            operation_result == self.result
        })
    }
}

pub fn run() {
    let equations = parse_input("7");
    let part1 = part1(&equations);

    println!("\n\nday 7 results:");
    println!("part 1: {}", part1);

    let part2 = part2(&equations);
    println!("part 2: {}", part2);
}

fn part1(equations: &[Equation]) -> u64 {
    let ops = vec![Operation::Add, Operation::Multply];

    equations
        .iter()
        .filter(|equation| equation.is_valid(&ops))
        .map(|equation| equation.result)
        .sum()
}

fn part2(equations: &[Equation]) -> u64 {
    let ops = vec![Operation::Add, Operation::Multply, Operation::Concat];

    equations
        .iter()
        .filter(|equation| equation.is_valid(&ops))
        .map(|equation| equation.result)
        .sum()
}

fn parse_input(file: &str) -> Vec<Equation> {
    let mut equations: Vec<Equation> = vec![];

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            if let Some((result, values_str)) = line.split_once(':') {
                equations.push(Equation {
                    result: result.parse().unwrap(),
                    values: values_str
                        .trim()
                        .split(' ')
                        .map(|x| x.parse().unwrap())
                        .collect(),
                });
            }
        }
    }

    equations
}

fn get_combos(count: usize, ops: &[Operation]) -> Vec<Vec<&Operation>> {
    if count <= 1 {
        return ops.iter().map(|op| vec![op]).collect();
    }
    let smaller_combos = get_combos(count - 1, ops);

    let mut ret: Vec<Vec<&Operation>> = Vec::with_capacity(ops.len());

    for op in ops {
        let combos = smaller_combos.clone();
        for mut combo in combos {
            combo.push(op);
            ret.push(combo);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = parse_input("7_sample");

        let part1 = part1(&sample);
        assert_eq!(3749, part1);

        let part2 = part2(&sample);
        assert_eq!(11387, part2);
    }

    #[test]
    fn combination_testing() {
        let ops = &[Operation::Add, Operation::Multply];
        let long_equation = Equation {
            values: vec![3, 42, 2, 4, 3, 258, 703, 4, 8],
            result: 1839056,
        };

        let longer_equation = Equation {
            values: vec![2, 9, 5, 9, 9, 4, 3, 7, 44, 5, 8, 7],
            result: 26205,
        };

        assert!(long_equation.is_valid(ops));
        assert!(longer_equation.is_valid(ops));
        // for combo in get_combos(3, ops) {
        //     println!("{:?}", combo);
        // }
    }
}
