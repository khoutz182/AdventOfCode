use std::{collections::VecDeque, fmt::Debug, str::FromStr};

use crate::utils::read_all_lines;

type Point = (u64, u64);

fn parse_input<T>(file: &str) -> Vec<(T, T)>
where
    T: FromStr,
    T::Err: Debug,
{
    read_all_lines(file)
        .iter()
        .map(|l| l.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<T>().unwrap(), y.parse::<T>().unwrap()))
        .collect()
}

fn calc_area(p1: &Point, p2: &Point) -> u64 {
    (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)
}

fn find_max_area(points: &[Point]) -> u64 {
    let mut max_area = 0_u64;
    for a in points {
        for b in points {
            max_area = calc_area(a, b).max(max_area);
        }
    }
    max_area
}

fn point_in_rect(point: &Point, rectangle: (&Point, &Point)) -> bool {
    if point == rectangle.0 || point == rectangle.1 {
        return false;
    }
    let range_x = rectangle.0.0.min(rectangle.1.0) + 1..rectangle.0.0.max(rectangle.1.0);
    let range_y = rectangle.0.1.min(rectangle.1.1) + 1..rectangle.0.1.max(rectangle.1.1);

    range_x.contains(&point.0) && range_y.contains(&point.1)
}

fn get_options(start: &Point, points: &[Point]) -> Vec<Point> {
    let mut options = vec![];
    for point in points.iter().filter(|p| start != *p) {
        if points
            .iter()
            .filter(|p| *p != point)
            .filter(|p| *p != start)
            .all(|p| !point_in_rect(p, (start, point)))
        {
            options.push(*point);
        } else {
            let invalid_points: Vec<_> = points
                .iter()
                .filter(|p| *p != point)
                .filter(|p| *p != start)
                .filter(|p| point_in_rect(p, (start, point)))
                .collect();
            println!(
                "for rectangle: {:?} - {:?}, invalid points found: {:?}",
                start, point, invalid_points
            );
        }
    }
    options
}

fn find_restricted_max_area(input: &[Point]) -> u64 {
    let mut max_area = 0_u64;
    let points: VecDeque<_> = input.iter().collect();

    // for a in points {
    //     for opt in get_options(a, points) {
    //         max_area = calc_area(a, &opt).max(max_area);
    //     }
    // }

    max_area
}

pub fn run() {
    let points = parse_input::<u64>("9");

    let max_area = find_max_area(&points);
    println!("max area: {max_area}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let points = parse_input::<u64>("9_sample");
        let max_area = find_max_area(&points);
        assert_eq!(50, max_area);

        let options = get_options(&(9, 7), &points);
        println!("options: {:?}", options);
        assert!(!options.is_empty());
        let restricted_area = find_restricted_max_area(&points);
        assert_eq!(24, restricted_area);
    }

    #[test]
    fn rectangle_bounds() {
        // for rectangle: (9, 7) - (11, 1), invalid points found: [(11, 7), (9, 5)]
        // assert!(!point_in_rect(&(11, 7), (&(9, 7), &(11, 1))));
        let rect = (&(2, 2), &(4, 4));
        assert!(point_in_rect(&(3, 3), rect));
        assert!(!point_in_rect(&(2, 2), rect));
        assert!(!point_in_rect(&(2, 4), rect));
        assert!(!point_in_rect(&(4, 2), rect));
        assert!(!point_in_rect(&(4, 4), rect));
    }
}
