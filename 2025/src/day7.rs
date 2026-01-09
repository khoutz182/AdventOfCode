use std::collections::HashMap;

use crate::utils::read_all_lines;

fn process_line(line: &str, beams: &mut HashMap<usize, usize>) -> u64 {
    if beams.is_empty() {
        let index: usize = line
            .char_indices()
            .filter(|(_, char)| *char == 'S')
            .map(|(idx, _)| idx)
            .next()
            .unwrap(); // cloudflare production ready
        beams.insert(index, 1);
        return 0;
    }

    let mut count = 0;
    for (idx, char) in line.char_indices() {
        if beams.contains_key(&idx) && char == '^' {
            let incoming = beams.remove(&idx).unwrap();
            // checking bounds doesnt seem necessary
            let first = idx - 1;
            let second = idx + 1;
            if let Some(val) = beams.get_mut(&first) {
                *val += incoming;
            } else {
                beams.insert(first, incoming);
            }
            if let Some(val) = beams.get_mut(&second) {
                *val += incoming;
            } else {
                beams.insert(second, incoming);
            }
            count += 1;
        }
    }

    count
}

fn get_split_count(lines: &Vec<String>) -> (u64, u64) {
    let mut beams: HashMap<usize, usize> = HashMap::new();
    let mut split_count = 0_u64;
    for line in lines {
        split_count += process_line(line, &mut beams);
    }
    let timelines = beams.values().sum::<usize>() as u64;

    (split_count, timelines)
}

pub fn run() {
    let input = read_all_lines("7");
    let (split_count, timelines) = get_split_count(&input);
    println!("part 1: {split_count}");
    println!("part 2: {timelines}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = read_all_lines("7_sample");
        let (split_count, timelines) = get_split_count(&input);
        // let min_input = &input[0..5];
        // let (split_count, timelines) = get_split_count(&min_input.to_vec());
        assert_eq!(21, split_count);
        assert_eq!(40, timelines);
    }

    #[test]
    fn starts_correctly() {
        let input = read_all_lines("7_sample");
        let mut beams = HashMap::new();
        let split_count = process_line(input.first().unwrap(), &mut beams);

        assert_eq!(Some(&1), beams.get(&7));
        assert_eq!(0, split_count);

        let size: usize = 0;
        let blah = size.checked_sub(1);
        println!("size - 1: {:?}", blah);
    }
}
