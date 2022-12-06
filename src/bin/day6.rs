use anyhow::Result;
use std::fs;

fn recursive_find_sequence(initial_idx: usize, string: &Vec<char>, len: usize) -> usize {
    let mut sequence = "".to_string();
    for i in initial_idx..string.len() {
        if !sequence.contains(string[i]) {
            sequence.push(string[i]);
            if sequence.len() == len {
                return i + 1;
            }
        } else {
            return recursive_find_sequence(initial_idx + 1, &string, len);
        }
    }
    0
}

fn main() -> Result<()> {
    let inputs = fs::read_to_string("./data/day6/input.txt").expect("Unable to read input file");
    let string = inputs.chars().collect::<Vec<char>>();

    println!(
        "The answer of the first problem is {}",
        recursive_find_sequence(0, &string, 4)
    );
    println!(
        "The answer of the second problem is {}",
        recursive_find_sequence(0, &string, 14)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::recursive_find_sequence;

    #[test]
    fn test_recursive_fn() {
        assert_eq!(
            recursive_find_sequence(0, &vec!['a', 'a', 'c', 'd', 'e', 'f', 'g'], 4),
            5
        );
    }
}
