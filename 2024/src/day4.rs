use crate::utils::read_lines;

pub fn run() {
    let grid = Grid::from("4");
    let x_locations = grid.all_x_locations();
    let count: usize = x_locations
        .iter()
        .map(|&(x, y)| grid.count_xmas(x, y))
        .sum();

    println!("count = {count}");
}

#[derive(Debug)]
struct Grid {
    bytes: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let bytes: Vec<Vec<u8>> = if let Ok(lines) = read_lines(value) {
            lines
                .map_while(Result::ok)
                .map(|row| row.bytes().collect())
                .collect()
        } else {
            vec![]
        };
        let (rows, cols) = (bytes.len(), bytes.first().map_or(0, |r| r.len()));

        Grid { bytes, rows, cols }
    }
}

impl Grid {
    fn all_x_locations(&self) -> Vec<(usize, usize)> {
        (0..self.rows)
            .flat_map(|row| (0..self.cols).map(move |col| (row, col)))
            .filter(|&(row, col)| self.bytes[row][col] == b'X')
            .collect()
    }

    fn get(&self, row: isize, col: isize) -> u8 {
        *self
            .bytes
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
            .unwrap_or(&b'_')
    }

    fn count_xmas(&self, row: usize, col: usize) -> usize {
        let dirs: Vec<(isize, isize)> = (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| (x, y)))
            .collect();

        dirs.iter()
            .filter(|(x, y)| {
                [b'M', b'A', b'S'].iter().enumerate().all(|(i, char)| {
                    let checked_row = row as isize + (x * (i as isize + 1));
                    let checked_col = col as isize + (y * (i as isize + 1));

                    let checked_char = self.get(checked_row, checked_col);

                    checked_char == *char
                })
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let grid = Grid::from("4_sample");
        let x_locations = grid.all_x_locations();
        let count: usize = x_locations
            .iter()
            .map(|&(x, y)| grid.count_xmas(x, y))
            .sum();

        assert_eq!(18, count);
    }
}
