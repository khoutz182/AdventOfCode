use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    // let input = "./inputs/day3_sample.txt";
    let input = "./inputs/day3.txt";

    let mut buffer: Vec<&String> = Vec::new();
    buffer.push(&String::from("................................................................................................................"));
    buffer.push(&String::from("................................................................................................................"));
    buffer.push(&String::from("................................................................................................................"));
    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(valid_line) = line {
                buffer.push(&valid_line);
                buffer.pop();
                print_buffer(buffer);

                
            }
        }
    } else {
        println!("{:?}", read_lines(input));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_buffer(buffer: Vec<&String>) {
    println!("{:?}", buffer);
}
