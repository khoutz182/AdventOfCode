use crate::utils::read_lines;

pub fn run() {
    if let Ok(lines) = read_lines("13") {
        for line in lines.map_while(Result::ok) {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = r#"
        blah
        "#;
    }
}
