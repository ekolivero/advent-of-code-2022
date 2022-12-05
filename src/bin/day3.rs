use std::fs;

fn cast_char_to_alphabet_order(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn solve_first(inputs: &String) -> u32 {
    let mut sum = 0;
    inputs.lines().for_each(|line| {
        let (s1, s2) = line.split_at(line.len() / 2);
        for char in s1.chars() {
            if s2.contains(char) {
                sum = sum + cast_char_to_alphabet_order(char);
                break;
            }
        }
    });
    sum
}

fn solve_second(inputs: &String) -> u32 {
    let mut sum = 0;
    let mut peakable_lines = inputs.lines().peekable();
    while peakable_lines.peek().is_some() {
        let s1 = peakable_lines.next().unwrap();
        let s2 = peakable_lines.next().unwrap();
        let s3 = peakable_lines.next().unwrap();

        for char in s1.chars() {
            if s2.contains(char) && s3.contains(char) {
                sum = sum + cast_char_to_alphabet_order(char);
                break;
            }
        }
    }
    sum
}

fn main() {
    let path = "./data/day3/input.txt";
    let inputs = fs::read_to_string(path).expect("Provide a valid input");

    println!("The answer of the first quiz is {}", solve_first(&inputs));
    println!("The answer of the second quiz is {}", solve_second(&inputs));
}

#[cfg(test)]
mod tests {
    use crate::{cast_char_to_alphabet_order, solve_first, solve_second};
    use std::fs;

    #[test]
    fn test_cast() {
        assert_eq!(cast_char_to_alphabet_order('a'), 1);
        assert_eq!(cast_char_to_alphabet_order('z'), 26);
        assert_eq!(cast_char_to_alphabet_order('A'), 27);
        assert_eq!(cast_char_to_alphabet_order('Z'), 52);
    }

    #[test]
    fn test_sample() {
        let path = "./data/day3/sample.txt";
        let inputs = fs::read_to_string(path).expect("Provide a valid input");
        assert_eq!(solve_first(&inputs), 157);
        assert_eq!(solve_second(&inputs), 70);
    }
}