use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Debug)]
struct Game {
    opponent: Shape,
    player: Shape,
}

impl Game {
    fn new(opponent: Shape, player: Shape) -> Game {
        Game { opponent, player }
    }
    fn play(&self) -> i32 {
        match *self {
            Game {
                opponent: Shape::Rock,
                player: Shape::Paper,
            } => 6,
            Game {
                opponent: Shape::Rock,
                player: Shape::Scissor,
            } => 0,
            Game {
                opponent: Shape::Paper,
                player: Shape::Rock,
            } => 0,
            Game {
                opponent: Shape::Paper,
                player: Shape::Scissor,
            } => 6,
            Game {
                opponent: Shape::Scissor,
                player: Shape::Rock,
            } => 6,
            Game {
                opponent: Shape::Scissor,
                player: Shape::Paper,
            } => 0,
            _ => 3,
        }
    }
}

fn get_shape(s: char) -> Shape {
    match s {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        _ => Shape::Scissor,
    }
}

fn get_player_shape(s: char) -> Shape {
    match s {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        _ => Shape::Scissor,
    }
}

fn get_player_shape_w_secret(opponent_shape: Shape, game_result: char) -> Shape {
    if game_result == 'X' {
        match opponent_shape {
            Shape::Paper => Shape::Rock,
            Shape::Rock => Shape::Scissor,
            Shape::Scissor => Shape::Paper,
        }
    } else if game_result == 'Z' {
        match opponent_shape {
            Shape::Paper => Shape::Scissor,
            Shape::Rock => Shape::Paper,
            Shape::Scissor => Shape::Rock,
        }
    } else {
        //This is the only part I don't really like.
        // I had to implement the Copy trait, otherwise &Shape != Shape.
        // Without reference the borrowchecker complain.
        opponent_shape
    }
}

fn solve_first_problem(path: &str) -> i32 {
    let inputs = fs::read_to_string(path).expect("Provide a valid input");

    let mut total_points = 0;

    inputs.lines().for_each(|line| {
        let game = Game::new(
            get_shape(line.chars().nth(0).unwrap()),
            get_player_shape(line.chars().nth(2).unwrap()),
        );

        total_points = total_points + game.play() + game.player as i32;
    });

    total_points
}

fn solve_second_problem(path: &str) -> i32 {
    let inputs = fs::read_to_string(path).expect("Provide a valid input");

    let mut total_points = 0;
    inputs.lines().for_each(|line| {
        let game_result = line.chars().nth(2).unwrap();
        let opponent_shape = get_shape(line.chars().nth(0).unwrap()).clone();
        let player_shape = get_player_shape_w_secret(opponent_shape, game_result);
        let game = Game::new(opponent_shape, player_shape);
        total_points = total_points + game.play() + player_shape as i32
    });

    total_points
}

fn main() {
    println!(
        "the score is {}",
        solve_first_problem("./data/day2/input.txt")
    );
    println!(
        "The score with the secret is {}",
        solve_second_problem("./data/day2/input.txt")
    );
}

#[cfg(test)]
mod tests {
    use crate::solve_first_problem;
    use crate::solve_second_problem;
    #[test]
    fn sample_test() {
        let path = "./data/day2/sample.txt";
        assert_eq!(solve_first_problem(path), 15);
    }
    #[test]
    fn aoc_first_input() {
        let path = "./data/day2/input.txt";
        assert_eq!(solve_first_problem(path), 11906)
    }
    #[test]
    fn sample_second_example() {
        let path = "./data/day2/sample.txt";
        assert_eq!(solve_second_problem(path), 12)
    }
    #[test]
    fn aoc_second_input() {
        let path = "./data/day2/input.txt";
        assert_eq!(solve_second_problem(path), 11186)
    }
}
