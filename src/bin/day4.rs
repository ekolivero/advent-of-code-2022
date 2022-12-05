use std::fs;

fn solve_first(inputs: &String) -> i32 {
    let mut sum = 0;
    inputs.lines().for_each(|line| {
        let mut splitted_elves = line.split(",");
        let first_elf = splitted_elves.next().unwrap();
        let second_elf = splitted_elves.next().unwrap();

        let mut first_elf_splitted = first_elf.split("-");
        let mut second_elf_splitted = second_elf.split("-");

        let (min, max) = (
            first_elf_splitted.next().unwrap().parse::<i32>().unwrap(),
            first_elf_splitted.next().unwrap().parse::<i32>().unwrap(),
        );

        let (min1, max1) = (
            second_elf_splitted.next().unwrap().parse::<i32>().unwrap(),
            second_elf_splitted.next().unwrap().parse::<i32>().unwrap(),
        );

        if (min <= min1 && max >= max1) || (min1 <= min && max1 >= max) {
            sum = sum + 1;
        }
    });

    sum
}

fn solve_second(inputs: &String) -> i32 {
    let mut sum = 0;
    inputs.lines().for_each(|line| {
        let mut splitted_elves = line.split(",");
        let first_elf = splitted_elves.next().unwrap();
        let second_elf = splitted_elves.next().unwrap();

        let mut first_elf_splitted = first_elf.split("-");
        let mut second_elf_splitted = second_elf.split("-");

        let (min, max) = (
            first_elf_splitted.next().unwrap().parse::<i32>().unwrap(),
            first_elf_splitted.next().unwrap().parse::<i32>().unwrap(),
        );

        let (min1, max1) = (
            second_elf_splitted.next().unwrap().parse::<i32>().unwrap(),
            second_elf_splitted.next().unwrap().parse::<i32>().unwrap(),
        );

        if (min <= min1 && max >= max1)
            || (min1 <= min && max1 >= max)
            || (min <= min1 && max >= min1)
            || (min1 <= min && max1 >= min)
        {
            sum = sum + 1;
        }
    });

    sum
}

fn main() {
    let inputs = fs::read_to_string("./data/day4/input.txt").expect("Unable to read input file");
    println!("The answer of the first quiz is {}", solve_first(&inputs));
    println!("The answer of the second quiz is {}", solve_second(&inputs));
}

#[cfg(test)]
mod tests {
    use crate::{solve_first, solve_second};
    use std::fs;

    #[test]
    fn first_sample() {
        let inputs =
            fs::read_to_string("./data/day4/sample.txt").expect("Unable to read input file");

        assert_eq!(solve_first(&inputs), 2);
    }

    #[test]
    fn second_sample() {
        let inputs =
            fs::read_to_string("./data/day4/sample.txt").expect("Unable to read input file");

        assert_eq!(solve_second(&inputs), 4);
    }
}
