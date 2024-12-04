use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines(day: u8) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(Path::new(&format!("./inputs/day{day}.txt")))?;
    Ok(io::BufReader::new(file).lines())
}
