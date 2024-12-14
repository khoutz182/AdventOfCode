use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads lines from the inputs folder. Assumes files are type ".txt" and have a prefix of "day".
///
/// # Examples
///
/// ```
/// // read lines from "./inputs/day5.txt"
/// read_lines("5");
/// // read lines from "./inputs/day5_sample.txt"
/// read_lines("5_sample");
/// ```
pub fn read_lines(day: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(Path::new(&format!("./inputs/day{day}.txt")))?;
    Ok(io::BufReader::new(file).lines())
}
