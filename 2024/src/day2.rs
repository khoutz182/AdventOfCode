use crate::utils::read_lines;

pub fn run() {
    let mut reports: Vec<Vec<u16>> = vec![];
    if let Ok(lines) = read_lines("2") {
        for line in lines.map_while(Result::ok) {
            reports.push(
                line.split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect(),
            );
        }
    }
    println!("safe reports: {}", count_safe(&reports, false));
    println!("safe + tolerant reports: {}", count_safe(&reports, true));
}

fn count_safe(reports: &[Vec<u16>], tolerant: bool) -> usize {
    let strict_cnt = reports.iter().filter(|report| is_safe(report)).count();
    let tolerant_cnt = if tolerant {
        reports
            .iter()
            .filter(|report| !is_safe(report))
            .filter(|report| is_tolerant_safe(report))
            .count()
    } else {
        0
    };

    strict_cnt + tolerant_cnt
}

fn is_tolerant_safe(report: &[u16]) -> bool {
    let vec_report = report.to_vec();
    (0..report.len())
        .map(|idx| {
            let mut clone = vec_report.clone();
            clone.remove(idx);

            clone
        })
        .any(|modified_report| is_safe(&modified_report))
}

fn is_safe(report: &[u16]) -> bool {
    is_sorted(report) && is_gradual(report)
}

fn is_sorted(report: &[u16]) -> bool {
    let sorted = report.is_sorted() || report.is_sorted_by(|a, b| a >= b);
    sorted && has_distinct(report)
}

fn has_distinct(report: &[u16]) -> bool {
    report.windows(2).all(|x| x[0] != x[1])
}

fn is_gradual(report: &[u16]) -> bool {
    report.windows(2).all(|x| x[0].abs_diff(x[1]) <= 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample: Vec<Vec<u16>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(2, count_safe(&sample, false), "ya dun goofed");
        assert_eq!(4, count_safe(&sample, true), "tolerance is not good");
    }
}
