use std::fs;

fn main() {

    let contents = fs::read_to_string("data/day1/input.txt")
        .expect("Provide a valid input");

    let mut elves_calories = vec![];
    let mut temp_number = 0;

    contents
        .lines().for_each(|line| {
            if line.is_empty() {
                elves_calories.push(temp_number);
                temp_number = 0;
            } else {
                temp_number = temp_number + line.parse::<i64>().unwrap();
            }
        });

    elves_calories.sort();
    let sum: i64 = elves_calories.iter().rev().take(3).sum();
    let elf_most_calories = elves_calories.last().unwrap();

    println!("The elf with the most calories is carrying: {}", *elf_most_calories);
    println!("Total calories {}", sum);
}

