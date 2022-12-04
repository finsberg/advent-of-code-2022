use std::fs;
enum Choice {
    X,
    Y,
    Z,
}

enum Hand {
    A,
    B,
    C,
}

/// Mapping from choice to hand for part 2
fn choice2hand_part2(choice: &Choice, opponent: &Hand) -> Hand {
    match choice {
        Choice::X => match opponent {
            // loose
            Hand::A => Hand::C,
            Hand::B => Hand::A,
            Hand::C => Hand::B,
        },
        Choice::Y => match opponent {
            // draw
            Hand::A => Hand::A,
            Hand::B => Hand::B,
            Hand::C => Hand::C,
        },
        Choice::Z => match opponent {
            // win
            Hand::A => Hand::B,
            Hand::B => Hand::C,
            Hand::C => Hand::A,
        },
    }
}

/// Returns the score for a given choice for part 2
fn outcome_score_part2(choice: &Choice) -> u32 {
    match choice {
        Choice::X => 0,
        Choice::Y => 3,
        Choice::Z => 6,
    }
}

/// Compute score based on selection for part 2
fn selection_score_part2(hand: &Hand) -> u32 {
    match hand {
        Hand::A => 1,
        Hand::B => 2,
        Hand::C => 3,
    }
}

/// Method for computing the score for part 2 of a single game
/// given the opponent and your choice as strings
fn compute_score_part2(opponent_str: &str, you_str: &str) -> u32 {
    let your_choice = you_from_str(you_str).unwrap();
    let opponent = opponent_from_str(opponent_str).unwrap();
    let you = choice2hand_part2(&your_choice, &opponent);

    let selection_score = selection_score_part2(&you);
    let outcome_score = outcome_score_part2(&your_choice);
    selection_score + outcome_score
}

/// Method for converting a string to a Choice enum
fn you_from_str(input: &str) -> Result<Choice, &'static str> {
    match input {
        "X" => Ok(Choice::X),
        "Y" => Ok(Choice::Y),
        "Z" => Ok(Choice::Z),
        _ => Err("Invalid string for you {input}"),
    }
}

/// Method for converting a string to an Hand Enum
fn opponent_from_str(input: &str) -> Result<Hand, &'static str> {
    match input {
        "A" => Ok(Hand::A),
        "B" => Ok(Hand::B),
        "C" => Ok(Hand::C),
        _ => Err("Invalid string for opponent {input}"),
    }
}

/// Compute score based on outcome for part 1
fn outcome_score_part1(opponent: &Hand, you: &Choice) -> u32 {
    match opponent {
        Hand::A => match you {
            Choice::X => 3,
            Choice::Y => 6,
            Choice::Z => 0,
        },
        Hand::B => match you {
            Choice::X => 0,
            Choice::Y => 3,
            Choice::Z => 6,
        },
        Hand::C => match you {
            Choice::X => 6,
            Choice::Y => 0,
            Choice::Z => 3,
        },
    }
}

/// Compute score based on selection for part 1
fn selection_score_part1(choice: &Choice) -> u32 {
    match choice {
        Choice::X => 1,
        Choice::Y => 2,
        Choice::Z => 3,
    }
}

/// Method for computing the score for part 1 of a single game
/// given the opponent and your choice as strings
fn compute_score_part1(opponent_str: &str, you_str: &str) -> u32 {
    let you = you_from_str(you_str).unwrap();
    let opponent = opponent_from_str(opponent_str).unwrap();

    let selection_score = selection_score_part1(&you);
    let outcome_score = outcome_score_part1(&opponent, &you);
    selection_score + outcome_score
}

/// Helper function to compute the score.
/// This method takes one string and split it based space
/// and uses the first letter as the opponent and the second
/// as your choice.
fn compute_score_helper(game: &str, compute_score: fn(&str, &str) -> u32) -> u32 {
    let mut iter = game.splitn(2, ' ');
    let opponent = iter.next().unwrap();
    let you = iter.next().unwrap();
    compute_score(opponent, you)
}

/// Compute the total score
fn compute_total(text: &str, compute_score: fn(&str, &str) -> u32) -> u32 {
    let games = text
        .trim()
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty());

    let scores = games.map(|game| compute_score_helper(game, compute_score));
    scores.sum()
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let score_part_1 = compute_total(&contents, compute_score_part1);
    println!("Part1: {score_part_1}");
    let score_part_2 = compute_total(&contents, compute_score_part2);
    println!("Part2: {score_part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_total_part1() {
        let example_input = "A Y\nB X\nC Z\n";
        let total = compute_total(example_input, compute_score_part1);
        assert_eq!(total, 15);
    }
    #[test]
    fn test_compute_score_part1() {
        assert_eq!(compute_score_part1("A", "Y"), 8);
        assert_eq!(compute_score_part1("B", "X"), 1);
        assert_eq!(compute_score_part1("C", "Z"), 6);
    }

    #[test]
    fn test_compute_total_part2() {
        let example_input = "A Y\nB X\nC Z\n";
        let total = compute_total(example_input, compute_score_part2);
        assert_eq!(total, 12);
    }

    #[test]
    fn test_compute_score_part2() {
        assert_eq!(compute_score_part2("A", "Y"), 4);
        assert_eq!(compute_score_part2("B", "X"), 1);
        assert_eq!(compute_score_part2("C", "Z"), 7);
    }
}
