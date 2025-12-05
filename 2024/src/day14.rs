use std::{collections::HashMap, ops::Range};

use crate::utils::read_lines;

#[derive(Debug)]
struct Robot {
    velocity: (i32, i32),
    position: (usize, usize),
}

#[derive(Debug)]
struct Grid {
    robots: Vec<Robot>,
    dimensions: (usize, usize),
}

impl Grid {
    fn new(dimensions: (usize, usize), lines: Vec<String>) -> Self {
        let mut robots: Vec<Robot> = Vec::new();
        lines
            .iter()
            .for_each(|l| robots.push(Robot::from(l.clone())));

        Grid { robots, dimensions }
    }

    fn step(&mut self, times: i32) {
        self.robots
            .iter_mut()
            .for_each(|r| r.step(self.dimensions, times));
    }

    fn quadrant_score(&self, ranges: (Range<usize>, Range<usize>)) -> usize {
        let map = self.count_map(); // Optimization? i hardly know her
        let mut sum: usize = 0;
        ranges.0.for_each(|x| {
            ranges.1.clone().for_each(|y| {
                if let Some(count) = map.get(&(x, y)) {
                    sum += *count
                }
            })
        });

        sum
    }

    fn safety_factor(&self) -> usize {
        let q1 = (
            (0..self.dimensions.0.checked_div(2).unwrap()),
            (0..self.dimensions.1.checked_div(2).unwrap()),
        );
        let q2 = (
            (self.dimensions.0.div_ceil(2)..self.dimensions.0),
            (0..self.dimensions.1.checked_div(2).unwrap()),
        );
        let q3 = (
            (0..self.dimensions.0.checked_div(2).unwrap()),
            (self.dimensions.1.div_ceil(2)..self.dimensions.1),
        );
        let q4 = (
            (self.dimensions.0.div_ceil(2)..self.dimensions.0),
            (self.dimensions.1.div_ceil(2)..self.dimensions.1),
        );

        let q1_sum = self.quadrant_score(q1);
        let q2_sum = self.quadrant_score(q2);
        let q3_sum = self.quadrant_score(q3);
        let q4_sum = self.quadrant_score(q4);

        println!("sums: {q1_sum}, {q2_sum}, {q3_sum}, {q4_sum}");

        q1_sum * q2_sum * q3_sum * q4_sum
    }

    fn count_map(&self) -> HashMap<(usize, usize), usize> {
        let mut map: HashMap<(usize, usize), usize> = HashMap::new();
        self.robots.iter().for_each(|r| {
            map.entry(r.position).and_modify(|x| *x += 1).or_insert(1);
        });

        map
    }

    fn print(&self) {
        let map = self.count_map();
        let longest_block = self.longest_block();

        println!("printing grid, longest block: {longest_block}");
        (0..self.dimensions.1).for_each(|j| {
            (0..self.dimensions.0).for_each(|i| {
                if let Some(count) = map.get(&(i, j)) {
                    print!("{count}");
                } else {
                    print!(".");
                }
            });
            println!();
        });
    }

    fn longest_block(&self) -> usize {
        let map = self.count_map();
        let mut max_length: usize = 0;
        let mut length: usize = 0;
        let mut block: bool = false;

        (0..self.dimensions.1).for_each(|j| {
            (0..self.dimensions.0).for_each(|i| {
                if map.contains_key(&(i, j)) {
                    length += 1;
                    block = true;
                } else {
                    if length > max_length {
                        max_length = length;
                    }
                    length = 0;
                    block = false;
                }
            })
        });

        max_length
    }
}

impl Robot {
    fn step(&mut self, grid_size: (usize, usize), times: i32) {
        let move_x = self.velocity.0 * times;
        let move_y = self.velocity.1 * times;
        let pre_x = self.position.0 as i32 + move_x;
        let pre_y = self.position.1 as i32 + move_y;
        let mod_x = pre_x % grid_size.0 as i32;
        let mod_y = pre_y % grid_size.1 as i32;
        let fix_x: usize = if mod_x < 0 {
            grid_size.0 - mod_x.unsigned_abs() as usize
        } else {
            mod_x as usize
        };
        let fix_y: usize = if mod_y < 0 {
            grid_size.1 - mod_y.unsigned_abs() as usize
        } else {
            mod_y as usize
        };

        self.position = (fix_x, fix_y);
    }
}

impl From<String> for Robot {
    fn from(value: String) -> Self {
        let (pos, vel) = value.split_once(" ").unwrap();

        let position: (usize, usize) = pos[2..]
            .split_once(",")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        let velocity: (i32, i32) = vel[2..]
            .split_once(",")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();

        Robot { position, velocity }
    }
}

pub fn run() {
    let mut grid = if let Ok(lines) = read_lines("14") {
        let all_lines: Vec<String> = lines.map_while(Result::ok).collect();
        Grid::new((101, 103), all_lines)
    } else {
        panic!("aaaahhhh");
    };

    // grid.step(100);
    // let safety_factor = grid.safety_factor();
    // println!("safety factor for part 1: {safety_factor}");
    let mut step_count = 0;

    loop {
        grid.step(1);
        step_count += 1;
        let longest = grid.longest_block();
        if longest > 15 {
            grid.print();
            print!("steps: {step_count}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14() {
        let robot = Robot::from("p=0,4 v=3,-3".to_string());
        assert_eq!((0, 4), robot.position);
        assert_eq!((3, -3), robot.velocity);

        // let mut sample_grid = Grid::new((11, 7), vec!["p=2,4 v=2,-3".to_string()]);
        //
        // sample_grid.print();
        // sample_grid.step(5);
        // smple_grid.print();

        let mut grid = Grid::new(
            (11, 7),
            [
                "p=0,4 v=3,-3",
                "p=6,3 v=-1,-3",
                "p=10,3 v=-1,2",
                "p=2,0 v=2,-1",
                "p=0,0 v=1,3",
                "p=3,0 v=-2,-2",
                "p=7,6 v=-1,-3",
                "p=3,0 v=-1,-2",
                "p=9,3 v=2,3",
                "p=7,3 v=-1,2",
                "p=2,4 v=2,-3",
                "p=9,5 v=-3,-3",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
        grid.print();
        grid.step(100);
        grid.print();
        println!("score: {}", grid.safety_factor());
    }
}
