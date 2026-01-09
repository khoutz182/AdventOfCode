use std::{collections::HashSet, fmt::Display};

use crate::utils::read_all_lines;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug)]
struct Circuit {
    positions: HashSet<Position>,
}

impl From<&String> for Position {
    fn from(value: &String) -> Self {
        let mut values = value.split(',').map(|s| s.parse::<u64>().unwrap());
        let x = values.next().unwrap();
        let y = values.next().unwrap();
        let z = values.next().unwrap();

        Self { x, y, z }
    }
}

impl Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display: Vec<String> = self
            .positions
            .iter()
            .map(|p| format!("[{}, {}, {}]\n", p.x, p.y, p.z))
            .collect();

        write!(f, "{}", display.join(", "))
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[x: {}, y: {}, z: {}]", self.x, self.y, self.z)
    }
}

impl Position {
    fn distance(&self, other: &Position) -> f64 {
        let x_diff = self.x.abs_diff(other.x).pow(2);
        let y_diff = self.y.abs_diff(other.y).pow(2);
        let z_diff = self.z.abs_diff(other.z).pow(2);

        f64::sqrt((x_diff + y_diff + z_diff) as f64)
    }
}

impl Circuit {
    fn new(position_0: Position, position_1: Position) -> Self {
        let mut positions = HashSet::new();
        positions.insert(position_0);
        positions.insert(position_1);

        Self { positions }
    }

    fn contains(&self, position: &Position) -> bool {
        self.positions.contains(position)
    }

    fn combine(&mut self, circuit: Circuit) {
        for position in circuit.positions {
            self.positions.insert(position);
        }
    }

    fn add_position(&mut self, position: Position) {
        self.positions.insert(position);
    }

    fn shortest_distance(&self, position: &Position) -> f64 {
        self.positions
            .iter()
            .map(|p| p.distance(position))
            .min_by(|a, b| a.total_cmp(b))
            .unwrap()
    }
}

fn closest<'a>(position: &'a Position, positions: &'a [Position]) -> (&'a Position, &'a Position) {
    let closest = positions
        .iter()
        .filter(|p| *p != position)
        .min_by_key(|p| position.distance(p) as u64)
        .unwrap();
    (position, closest)
}

fn build_shortest_circuits(positions: Vec<Position>) -> Vec<Circuit> {
    let mut circuits: Vec<Circuit> = vec![];

    let mut pairs: Vec<_> = positions.iter().map(|p| closest(p, &positions)).collect();
    pairs.sort_by(|a, b| a.0.distance(a.1).total_cmp(&(b.0.distance(b.1))));

    for pair in &pairs {
        println!(
            "pair: {}, {}: distance: {}",
            pair.0,
            pair.1,
            pair.0.distance(pair.1)
        );
    }

    for (a, b) in pairs {
        if let Some(circuit) = circuits.iter_mut().find(|c| c.contains(a) || c.contains(b)) {
            circuit.add_position(a.clone());
            circuit.add_position(b.clone());
        } else {
            circuits.push(Circuit::new(a.clone(), b.clone()));
        }
    }

    // for position in &positions {
    //     let closest = positions
    //         .iter()
    //         .filter(|p| *p != position)
    //         .min_by_key(|p| position.distance(p) as u64)
    //         .unwrap();
    //
    //     println!("closest to {position} is {closest}");
    //     if let Some(circuit) = circuits.iter_mut().find(|c| c.contains(closest)) {
    //         circuit.add_position(position.clone());
    //     } else {
    //         circuits.push(Circuit::new(position.clone()));
    //     }
    // }

    // let positions: Vec<Position> = circuits
    //     .iter()
    //     .filter(|c| c.positions.len() == 1)
    //     .map(|c| c.positions.iter().collect::<Vec<&Position>>())
    //     .map(|p| (*p.first().unwrap()).clone())
    //     .collect();
    //
    // for position in positions {
    //     let closest = circuits
    //         .iter_mut()
    //         .filter(|c| c.positions.len() > 1)
    //         .min_by(|a, b| {
    //             a.shortest_distance(&position)
    //                 .total_cmp(&b.shortest_distance(&position))
    //         })
    //         .unwrap();
    //     println!("adding {:?} to {closest}", position);
    //     closest.add_position(position);
    // }
    // circuits.retain(|c| c.positions.len() > 1);
    circuits.sort_by_key(|c| c.positions.len());
    circuits.reverse();

    circuits
}

pub fn run() {
    let lines = read_all_lines("8");
    let positions: Vec<Position> = lines.iter().map(Position::from).collect();
    let circuits = build_shortest_circuits(positions);
    let top_3 = circuits
        .iter()
        .take(3)
        .map(|c| c.positions.len())
        .product::<usize>();

    println!("top 3: {top_3}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "it doesnt work yet"]
    fn sample() {
        let lines = read_all_lines("8_sample");
        let positions: Vec<Position> = lines.iter().map(Position::from).collect();
        let circuits = build_shortest_circuits(positions);
        // for circ in &circuits {
        //     println!("circuit: {circ}");
        // }
        let answer: usize = circuits.iter().take(3).map(|c| c.positions.len()).product();
        assert_eq!(40, answer);
    }
}
