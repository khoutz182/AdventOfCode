use std::collections::HashSet;

struct Grid {
    size: u16,
    corrupt: HashSet<(u16, u16)>,
}

impl Grid {
    fn new(size: u16, corrupt_lines: Vec<String>) -> Grid {
        let corrupt: HashSet<(u16, u16)> = corrupt_lines
            .iter()
            .map(|l| l.split_once(",").unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();

        Grid { size, corrupt }
    }
}

pub fn run() {}
