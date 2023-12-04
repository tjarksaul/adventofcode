use std::collections::VecDeque;

fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let total_points = calculate_total_points(&input);
    let total_cards = calculate_total_cards(&input);

    dbg!(total_points, total_cards);
}

fn calculate_total_points(input: &Vec<Card>) -> usize {
    input.iter().fold(0, |acc, card| 
        acc + card.numbers.iter().fold(0, |score, num| {
            let contains = card.winning.contains(num);
            if contains {
                let mut score = score;
                if score == 0 {
                    score = 1;
                } else {
                    score = score * 2;
                }
                return score;
            }
            return score;
        })
    )
}

fn calculate_total_cards(input: &Vec<Card>) -> usize {
    let clone: Vec<Card> = input.iter().map(|c| c.copy()).collect();
    let mut queue = VecDeque::from(clone);
    let mut amount = input.len();

    while !queue.is_empty() {
        let card: Card = queue.pop_front().unwrap();
        let winning = card.numbers.iter().filter(|&num| card.winning.contains(num)).count();
        amount += winning;
        for idx in 0..winning {
            queue.push_back(input[card.id + idx].copy());
        }
    }

    amount
}

#[derive(Clone)]
pub struct Card {
    id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn copy(&self) -> Card {
        Card { 
            id: self.id, 
            winning: self.winning.iter().map(|w| *w).collect(), 
            numbers: self.numbers.iter().map(|w| *w).collect() 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Card> {
        vec![
            Card { id: 1, winning: vec![41, 48, 83, 86, 17], numbers: vec![83, 86,  6, 31, 17,  9, 48, 53] },
            Card { id: 2, winning: vec![13, 32, 20, 16, 61], numbers: vec![61, 30, 68, 82, 17, 32, 24, 19] },
            Card { id: 3, winning: vec![ 1, 21, 53, 59, 44], numbers: vec![69, 82, 63, 72, 16, 21, 14,  1] },
            Card { id: 4, winning: vec![41, 92, 73, 84, 69], numbers: vec![59, 84, 76, 51, 58,  5, 54, 83] },
            Card { id: 5, winning: vec![87, 83, 26, 28, 32], numbers: vec![88, 30, 70, 12, 93, 22, 82, 36] },
            Card { id: 6, winning: vec![31, 18, 13, 56, 72], numbers: vec![74, 77, 10, 23, 35, 67, 36, 11] },
        ]
    }

    #[test]
    fn calculates_total_points_correctly() {
        let input = get_input();

        let total_points = super::calculate_total_points(&input);

        assert_eq!(total_points, 13);
    }

    #[test]
    fn caclulates_total_cards_correctly() {
        let input = get_input();

        let total_cards = super::calculate_total_cards(&input);

        assert_eq!(total_cards, 30);
    }
}

mod read {
    // Sample input:
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    use nom::{
        bytes::complete::tag, character::complete as cc, combinator::all_consuming, combinator::map,
        multi::separated_list1, sequence::tuple, sequence::delimited, Finish, IResult,
    };
    use super::Card;

    pub fn read_all_lines(i: &'static str) -> Vec<Card> {
        i.lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .collect()
    }

    fn parse_usize(i: &str) -> IResult<&str, usize> {
        map(cc::u32, |num: u32| num as usize)(i)
    }

    // 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    fn parse_line(i: &str) -> IResult<&str, Card> {
        let (i, (_, _, id, _)) = tuple((tag("Card"), cc::multispace1, parse_usize, tag(":")))(i)?;
        let (i, (winning, _, numbers)) = delimited(
            cc::multispace0,
            tuple((
                delimited(cc::multispace0, separated_list1(cc::multispace1, parse_usize), cc::multispace0), 
                tag("|"), 
                delimited(cc::multispace0, separated_list1(cc::multispace1, parse_usize), cc::multispace0),
            )),
            cc::multispace0
          )(i)?;
        Ok((i, Card { id, winning, numbers }))
    }
}
