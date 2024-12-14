use std::vec;

use crate::utils::read_lines;

#[allow(dead_code)]
pub fn run() {
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];

    if let Ok(lines) = read_lines("1") {
        for line in lines.map_while(Result::ok) {
            let mut parts = line.split_whitespace();
            list1.push(parts.next().expect("whoa whoa wee whoa").parse().unwrap());
            list2.push(parts.next().expect("whoa whoa we whoa").parse().unwrap());
        }
    } else {
        panic!("orrr noorrrr");
    }
    list1.sort();
    list2.sort();
    sum_of_distance(&list1, &list2);
    similarity_score(&list1, &list2);
}

fn sum_of_distance(list1: &[u32], list2: &[u32]) {
    let sum: u32 = list1
        .iter()
        .zip(list2)
        .map(|(l1, l2)| l1.abs_diff(*l2))
        .sum();
    println!("sum: {}", sum)
}

fn similarity_score(list1: &[u32], list2: &[u32]) {
    let similarity_sum: u32 = list1
        .iter()
        .map(|id| {
            id * list2
                .iter()
                .skip_while(|&y| y < id)
                .take_while(|&y| y == id)
                .count() as u32
        })
        .sum();

    println!("similarity sum: {}", similarity_sum)
}
