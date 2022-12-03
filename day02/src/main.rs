use std::{error::Error, fs};

#[derive(Debug, PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn parse_input(input: &str) -> Result<Vec<(Hand, Hand)>, Box<dyn Error>> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Result<(Hand, Hand), Box<dyn Error>> {
    if line.len() < 3 {
        return Err(Box::from(format!("Line '{line}' too short")));
    }

    let first_hand = match line.chars().nth(0).unwrap() {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        'C' => Hand::Scissors,
        _ => return Err(Box::from("Invalid first hand")),
    };

    let second_hand = match line.chars().nth(2).unwrap() {
        'X' => Hand::Rock,
        'Y' => Hand::Paper,
        'Z' => Hand::Scissors,
        _ => return Err(Box::from("Invalid second hand")),
    };

    Ok((first_hand, second_hand))
}

fn points_for_round(opponent_hand: &Hand, my_hand: &Hand) -> u32 {
    let points = points_for_victory(opponent_hand, my_hand);

    match my_hand {
        Hand::Rock => points + 1,
        Hand::Paper => points + 2,
        Hand::Scissors => points + 3,
    }
}

fn points_for_victory(opponent_hand: &Hand, my_hand: &Hand) -> u32 {
    use Hand::*;

    match (opponent_hand, my_hand) {
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
        _ => 6,
    }
}

fn parse_input_2(input: &str) -> Result<Vec<(Hand, Hand)>, Box<dyn Error>> {
    input.lines().map(|line| parse_line_2(line)).collect()
}

fn parse_line_2(line: &str) -> Result<(Hand, Hand), Box<dyn Error>> {
    if line.len() < 3 {
        return Err(Box::from(format!("Line '{line}' too short")));
    }

    let first_hand = match line.chars().nth(0).unwrap() {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        'C' => Hand::Scissors,
        _ => return Err(Box::from("Invalid first hand")),
    };

    let second_hand = find_corresponding_hand(&first_hand, line.chars().nth(1).unwrap())?;

    Ok((first_hand, second_hand))
}

fn find_corresponding_hand(
    opponent_hand: &Hand,
    how_round_ends: char,
) -> Result<Hand, Box<dyn Error>> {
    use Hand::*;

    match how_round_ends {
        'X' => {
            // I need to lose
            match opponent_hand {
                Rock => Ok(Scissors),
                Paper => Ok(Rock),
                Scissors => Ok(Paper),
            }
        }
        'Y' => {
            // I need to draw - same hand as opponent
            Ok(opponent_hand.clone())
        }
        'Z' => {
            // I need to win
            match opponent_hand {
                Rock => Ok(Paper),
                Paper => Ok(Scissors),
                Scissors => Ok(Rock),
            }
        }
        _ => Err(Box::from("Invalid way to end round")),
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();

    if let Ok(hands) = parse_input_2(&input) {
        let total_points = hands
            .iter()
            .map(|(opponent_hand, my_hand)| points_for_round(opponent_hand, my_hand))
            .sum::<u32>();

        println!("{total_points}");
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, points_for_round};

    #[test]
    fn should_parse_input() {
        use crate::Hand::*;
        let input = "A Y\n\
            B X\n\
            C Z";

        let res = parse_input(input);
        assert!(res.is_ok());
        let games = res.unwrap();
        println!("{:?}", games);
        let mut games_iter = games.iter();
        assert_eq!(Some(&(Rock, Paper)), games_iter.next());
        assert_eq!(Some(&(Paper, Rock)), games_iter.next());
        assert_eq!(Some(&(Scissors, Scissors)), games_iter.next());
        assert_eq!(None, games_iter.next());
    }

    #[test]
    fn should_compute_points_correctly() {
        use crate::Hand::*;
        let round1 = (Rock, Paper);
        let round2 = (Paper, Rock);
        let round3 = (Scissors, Scissors);

        assert_eq!(8, points_for_round(&round1.0, &round1.1));
        assert_eq!(1, points_for_round(&round2.0, &round2.1));
        assert_eq!(6, points_for_round(&round3.0, &round3.1));
    }
}
