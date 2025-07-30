use crate::utils::read_lines;
use regex::Regex;

pub fn run() {
    if let Ok(lines) = read_lines("3") {
        let mut sum: i32 = 0;
        let mut sum_p2: i32 = 0;
        let mut enabled = true;
        for line in lines.map_while(Result::ok) {
            sum += do_math(line.clone());
            let (sum, still_enabled) = do_conditional_math(line, enabled);
            sum_p2 += sum;
            enabled = still_enabled;
        }
        println!("total: {sum}");
        println!("conditional total: {sum_p2}");
    }
}

fn do_math(input: String) -> i32 {
    let expr = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    expr.captures_iter(input.as_str())
        .map(|c| {
            let (_, [first, second]) = c.extract();

            first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap()
        })
        .sum()
}

fn do_conditional_math(input: String, mut starting_state: bool) -> (i32, bool) {
    let expr = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don\'t\(\)|do\(\)").unwrap();
    let mul_expr = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total: i32 = 0;

    expr.find_iter(input.as_str())
        .for_each(|m| match m.as_str() {
            "don't()" => {
                starting_state = false;
            }
            "do()" => {
                starting_state = true;
            }
            _ => {
                if starting_state {
                    let (_, [first, second]) = mul_expr.captures(m.as_str()).unwrap().extract();
                    let a = first.parse::<i32>().unwrap();
                    let b = second.parse::<i32>().unwrap();
                    // println!("a={a}, b={b}");
                    total += a * b;
                }
            }
        });

    (total, starting_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = do_math(example.to_string());

        assert_eq!(result, 161);

        let p2_example =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()don't()mul(8,5)mul(1,2)";

        let (p2_result, _) = do_conditional_math(p2_example.to_string(), true);
        assert_eq!(p2_result, 48);
    }
}
