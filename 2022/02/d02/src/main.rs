use std::collections::HashMap;
use std::str::FromStr;
mod read;

fn main() {
    let input = read::read_input("input.txt".to_string(), row_parser_1);
    let total_score = calculate_scores(input);

    let input_2 = read::read_input("input.txt".to_string(), row_parser_2);
    let total_score_2 = calculate_scores_2(input_2);

    println!("Total score: {}", total_score);
    println!("Total with second strategy: {}", total_score_2);
}

const SCORE_WIN: i64 = 6;
const SCORE_DRAW: i64 = 3;
const SCORE_LOSS: i64 = 0;

fn row_parser_1(str: &str) -> Play {
    let mut splits = str.split(" ");
    let opponent = splits.next().unwrap();
    let player = splits.next().unwrap();

    return Play {
        opponent: Move::from_str(opponent).unwrap(),
        player: Move::from_str(player).unwrap(),
    };
}

fn row_parser_2(str: &str) -> ExpectedPlay {
    let mut splits = str.split(" ");
    let opponent = splits.next().unwrap();
    let expected_result = splits.next().unwrap();

    return ExpectedPlay {
        opponent: Move::from_str(opponent).unwrap(),
        expected_result: PlayResult::from_str(expected_result).unwrap(),
    };
}

pub fn calculate_scores(vec: Vec<Play>) -> i64 {
    let score = vec.iter().map(|play| calculate_score(play)).fold(0, |a, b| a + b);

    return score;
}

pub fn calculate_scores_2(vec: Vec<ExpectedPlay>) -> i64 {
    let plays: Vec<Play> = vec.iter().map(|exp| calculate_play_from_expected_play(exp)).collect();

    return calculate_scores(plays);
}

pub fn calculate_score(play: &Play) -> i64 {
    let move_score = calculate_move_score(&play.player);
    let result = calculate_result(&play);

    return move_score + result;
}

pub fn calculate_move_score(mov: &Move) -> i64 {
    let score_map = HashMap::from([
        (Move::Rock, 1),
        (Move::Paper, 2),
        (Move::Scissors, 3),
    ]);

    return score_map[&mov];
}

pub fn calculate_result(play: &Play) -> i64 {
    if play.player == play.opponent {
        return SCORE_DRAW;
    }
    if play.player == Move::Rock && play.opponent == Move::Scissors 
        || play.player == Move::Paper && play.opponent == Move::Rock
        || play.player == Move::Scissors && play.opponent == Move::Paper {
        return SCORE_WIN;
    }
    return SCORE_LOSS;
}

fn calculate_play_from_expected_play(expected: &ExpectedPlay) -> Play {
    let loss_map = HashMap::from([
        (Move::Rock, Move::Scissors),
        (Move::Paper, Move::Rock),
        (Move::Scissors, Move::Paper),
    ]);
    let win_map = HashMap::from([
        (Move::Rock, Move::Paper),
        (Move::Paper, Move::Scissors),
        (Move::Scissors, Move::Rock),
    ]);

    if expected.expected_result == PlayResult::Draw {
        return Play {
            opponent: expected.opponent,
            player: expected.opponent,
        }
    }

    if expected.expected_result == PlayResult::Win {
        return Play {
            opponent: expected.opponent,
            player: win_map[&expected.opponent],
        }
    }

    return Play {
        opponent: expected.opponent,
        player: loss_map[&expected.opponent],
    }
}

pub struct Play {
    opponent: Move,
    player: Move,
}

pub struct ExpectedPlay {
    opponent: Move,
    expected_result: PlayResult,
}

#[derive(Eq)]
#[derive(PartialEq)]
pub enum PlayResult {
    Win, Loss, Draw,
}

impl FromStr for PlayResult {
    type Err = ();

    fn from_str(input: &str) -> Result<PlayResult, Self::Err> {
        match input {
            "X"  => Ok(PlayResult::Loss),
            "Y"  => Ok(PlayResult::Draw),
            "Z"  => Ok(PlayResult::Win),
            _      => Err(()),
        }
    }
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Copy, Clone)]
pub enum Move {
    Rock, Paper, Scissors,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "X" | "A"  => Ok(Move::Rock),
            "Y" | "B"  => Ok(Move::Paper),
            "Z" | "C"  => Ok(Move::Scissors),
            _      => Err(()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_move_score_correctly() {
        assert_eq!(calculate_move_score(&Move::Rock), 1);
        assert_eq!(calculate_move_score(&Move::Paper), 2);
        assert_eq!(calculate_move_score(&Move::Scissors), 3);
    }

    #[test]
    fn calculates_plays_correctly() {
        let loss = 0;
        let draw = 3;
        let win = 6;
        assert_eq!(calculate_result(&Play { opponent: Move::Rock, player: Move::Rock }), draw);
        assert_eq!(calculate_result(&Play { opponent: Move::Rock, player: Move::Paper }), win);
        assert_eq!(calculate_result(&Play { opponent: Move::Rock, player: Move::Scissors }), loss);
        assert_eq!(calculate_result(&Play { opponent: Move::Paper, player: Move::Rock }), loss);
        assert_eq!(calculate_result(&Play { opponent: Move::Paper, player: Move::Paper }), draw);
        assert_eq!(calculate_result(&Play { opponent: Move::Paper, player: Move::Scissors }), win);
        assert_eq!(calculate_result(&Play { opponent: Move::Scissors, player: Move::Rock }), win);
        assert_eq!(calculate_result(&Play { opponent: Move::Scissors, player: Move::Paper }), loss);
        assert_eq!(calculate_result(&Play { opponent: Move::Scissors, player: Move::Scissors }), draw);
    }

    #[test]
    fn calculates_score_correctly() {
        let input = vec![
            Play {
                opponent: Move::Rock,
                player: Move::Paper,
            },
            Play {
                opponent: Move::Paper,
                player: Move::Rock,
            },
            Play {
                opponent: Move::Scissors,
                player: Move::Scissors,
            },
        ];
        let score = calculate_scores(input);

        assert_eq!(score, 15);
    }

    #[test]
    fn calculates_expected_plays_correctly() {
        let input = vec![
            ExpectedPlay {
                opponent: Move::Rock,
                expected_result: PlayResult::Draw,
            },
            ExpectedPlay {
                opponent: Move::Paper,
                expected_result: PlayResult::Loss,
            },
            ExpectedPlay {
                opponent: Move::Scissors,
                expected_result: PlayResult::Win,
            },
        ];

        let score = calculate_scores_2(input);

        assert_eq!(score, 12);
    }
}
