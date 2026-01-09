use std::mem::replace;

use crate::utils::read_lines;

struct Warehouse {
    grid: Vec<Vec<bool>>,
    rows: usize,
    columns: usize,
}

impl From<Vec<&str>> for Warehouse {
    fn from(value: Vec<&str>) -> Self {
        let mut grid: Vec<Vec<bool>> = Vec::new();
        let rows = value.len();
        let columns = value.first().unwrap().len();
        for line in value {
            grid.push(line.chars().map(|c| c == '@').collect());
        }

        Warehouse {
            grid,
            rows,
            columns,
        }
    }
}

impl Warehouse {
    fn get(&self, row: isize, column: isize) -> bool {
        // if row < 0 || column < 0 {
        //     return false;
        // }
        // if row >= self.rows || column >= self.columns {
        //     return false;
        // }
        *self
            .grid
            .get(row as usize)
            .and_then(|row| row.get(column as usize))
            .unwrap_or(&false)
    }

    fn get_adjacent_count(&self, row: isize, column: isize) -> usize {
        [
            (row - 1, column - 1),
            (row - 1, column),
            (row - 1, column + 1),
            (row, column - 1),
            (row, column + 1),
            (row + 1, column - 1),
            (row + 1, column),
            (row + 1, column + 1),
        ]
        .iter()
        .map(|(row, col)| self.get(*row, *col))
        .filter(|x| *x)
        .count()
    }

    fn count_accessible_rolls(&self) -> usize {
        let mut count: usize = 0;
        for row in 0..self.rows {
            for col in 0..self.columns {
                if self.get(row as isize, col as isize)
                    && self.get_adjacent_count(row as isize, col as isize) < 4
                {
                    count += 1;
                }
            }
        }

        count
    }

    fn remove(&mut self, row: usize, col: usize) {
        let line = self.grid.get_mut(row).unwrap();
        let _ = replace(&mut line[col], false);
    }

    fn remove_accessible_rolls(&mut self) -> usize {
        let mut count: usize = 0;
        for row in 0..self.rows {
            for col in 0..self.columns {
                if self.get(row as isize, col as isize)
                    && self.get_adjacent_count(row as isize, col as isize) < 4
                {
                    self.remove(row, col);
                    count += 1;
                }
            }
        }

        count
    }
}

pub fn run() {
    let lines: Vec<String> = read_lines("4").unwrap().map_while(Result::ok).collect();

    let mut warehouse = Warehouse::from(lines.iter().map(String::as_str).collect::<Vec<&str>>());
    let accessible = warehouse.count_accessible_rolls();

    println!("accessible: {accessible}");
    let mut removed = warehouse.remove_accessible_rolls();
    let mut total = removed;
    while removed > 0 {
        removed = warehouse.remove_accessible_rolls();
        total += removed;
    }

    println!("total removed: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];
        let mut warehouse = Warehouse::from(sample);
        assert!(!warehouse.get(-1, -2));
        assert!(!warehouse.get(0, 0));
        assert!(!warehouse.get(0, 1));
        assert!(warehouse.get(0, 2));

        assert_eq!(2, warehouse.get_adjacent_count(0, 0));
        assert_eq!(4, warehouse.get_adjacent_count(0, 1));
        assert_eq!(7, warehouse.get_adjacent_count(2, 1));

        assert_eq!(13, warehouse.count_accessible_rolls());
        let mut removed = warehouse.remove_accessible_rolls();
        let mut total = removed;
        while removed > 0 {
            removed = warehouse.remove_accessible_rolls();
            total += removed;
        }

        println!("removed a total of {total} rolls");
    }
}
