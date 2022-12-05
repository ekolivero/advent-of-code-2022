use std::fs;

fn parse_movement(line: &str) -> (usize, usize, usize) {
    let t: String = line.chars().filter(|c| c.is_digit(10)).collect();
    match t.len() {
        3 => (
            t[0..1].parse::<usize>().unwrap(),
            t[1..2].parse::<usize>().unwrap() - 1,
            t[2..3].parse::<usize>().unwrap() - 1,
        ),
        _ => (
            t[0..2].parse::<usize>().unwrap(),
            t[2..3].parse::<usize>().unwrap() - 1,
            t[3..4].parse::<usize>().unwrap() - 1,
        ),
    }
}

fn solve_first(crates: &mut Vec<String>, inputs: &String) -> String {
    let mut result: String = "".to_string();

    for line in inputs.lines() {
        if line.starts_with("move") {
            let (number, from, to) = parse_movement(line);

            for _ in 0..number {
                let value = crates[from as usize].pop().unwrap();
                crates[to as usize].push(value);
            }
        }
    }

    crates.into_iter().for_each(|c| {
        result.push_str(&c.chars().last().unwrap_or_default().to_string());
    });

    result
}

fn solve_second(crates: &mut Vec<String>, inputs: &String) -> String {
    let mut result: String = "".to_string();

    for line in inputs.lines() {
        if line.starts_with("move") {
            let (number, from, to) = parse_movement(line);

            let mut ordered_movements: String = "".to_string();

            for _ in 0..number {
                ordered_movements =
                    crates[from as usize].pop().unwrap().to_string() + &ordered_movements;
            }

            crates[to as usize].push_str(&ordered_movements);
        }
    }

    crates.into_iter().for_each(|c| {
        result.push_str(&c.chars().last().unwrap_or_default().to_string());
    });

    result
}

fn main() {
    let inputs = fs::read_to_string("./data/day5/input.txt").expect("Unable to read input file");
    let mut crates: Vec<String> = vec!["".to_string(); 9];

    for line in inputs.lines() {
        if line == "" {
            break;
        }
        for (i, c) in line.char_indices() {
            if c.is_ascii_alphabetic() {
                crates[i / 4].push(c);
            }
        }
    }

    let crates = crates
        .into_iter()
        .map(|c| c.chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    let result_first = solve_first(&mut crates.clone(), &inputs);
    let result_second: String = solve_second(&mut crates.clone(), &inputs);

    println!("Result first is: {}", result_first);
    println!("Result second is: {}", result_second);
}

#[cfg(test)]
mod tests {
    use crate::parse_movement;

    #[test]
    fn test_parse_movement() {
        assert_eq!(parse_movement("move 12 from 6 to 7"), (12, 5, 6));
        assert_eq!(parse_movement("move 1 from 1 to 2"), (1, 0, 1));
        assert_eq!(parse_movement("move 2 from 2 to 3"), (2, 1, 2));
        assert_eq!(parse_movement("move 3 from 3 to 4"), (3, 2, 3));
    }
}
