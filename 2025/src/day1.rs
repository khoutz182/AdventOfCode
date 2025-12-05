use crate::utils::read_lines;

struct Turn {
    amount: i32,
}

struct Playbook {
    turns: Vec<Turn>,
}

impl From<&str> for Turn {
    fn from(value: &str) -> Self {
        let (dir, amt) = value.split_at(1);
        let amount = amt.parse::<i32>().unwrap();
        let amount = match dir {
            "R" => amount,
            "L" => -amount,
            _ => panic!("aaahhhhh"),
        };

        Self { amount }
    }
}

impl From<Vec<String>> for Playbook {
    fn from(values: Vec<String>) -> Self {
        let turns = values.iter().map(|v| Turn::from(v.as_str())).collect();
        Self { turns }
    }
}

impl Playbook {
    fn play(&self) -> usize {
        let mut zeros: usize = 0;
        let mut current_position = 50;
        for turn in &self.turns {
            current_position = (current_position + turn.amount) % 100;
            if current_position == 0 {
                zeros += 1;
            }
        }

        zeros
    }

    fn play2(&self) -> usize {
        let mut zeros: usize = 0;
        let mut curr_pos = 50;
        for turn in &self.turns {
            let new_pos = curr_pos + turn.amount;
            if new_pos <= 0 && curr_pos != 0 {
                zeros += 1;
            }
            zeros += (new_pos / 100).unsigned_abs() as usize;
            curr_pos = new_pos.rem_euclid(100);
        }
        zeros
    }
}

pub fn run() {
    let lines: Vec<String> = read_lines("1").unwrap().map_while(Result::ok).collect();
    let playbook = Playbook::from(lines);
    let zeros = playbook.play();
    println!("zeros: {}", zeros);
    let zeros = playbook.play2();
    println!("zeros p2: {zeros}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn playbook(input: Vec<&str>) -> Playbook {
        Playbook::from(input.iter().map(|s| s.to_string()).collect::<Vec<String>>())
    }

    #[test]
    fn sample() {
        let playbook = playbook(vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]);
        let zeros = playbook.play();
        assert_eq!(3, zeros);

        let zeros = playbook.play2();
        assert_eq!(6, zeros);
    }

    #[test]
    fn playground() {
        let playbook = playbook(vec!["R50", "L100"]);
        let zeros = playbook.play2();
        assert_eq!(2, zeros);
    }
}
