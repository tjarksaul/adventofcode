use std::collections::HashSet;

fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let total_points = calculate_total_points(&input);

    dbg!(total_points);
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

pub struct Card {
    winning: HashSet<u32>,
    numbers: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_total_points_correctly() {
        let input = vec![
            Card { winning: HashSet::from_iter(vec![41, 48, 83, 86, 17].iter().cloned()), numbers: vec![83, 86,  6, 31, 17,  9, 48, 53] },
            Card { winning: HashSet::from_iter(vec![13, 32, 20, 16, 61].iter().cloned()), numbers: vec![61, 30, 68, 82, 17, 32, 24, 19] },
            Card { winning: HashSet::from_iter(vec![ 1, 21, 53, 59, 44].iter().cloned()), numbers: vec![69, 82, 63, 72, 16, 21, 14,  1] },
            Card { winning: HashSet::from_iter(vec![41, 92, 73, 84, 69].iter().cloned()), numbers: vec![59, 84, 76, 51, 58,  5, 54, 83] },
            Card { winning: HashSet::from_iter(vec![87, 83, 26, 28, 32].iter().cloned()), numbers: vec![88, 30, 70, 12, 93, 22, 82, 36] },
            Card { winning: HashSet::from_iter(vec![31, 18, 13, 56, 72].iter().cloned()), numbers: vec![74, 77, 10, 23, 35, 67, 36, 11] },
        ];

        let total_points = super::calculate_total_points(&input);

        assert_eq!(total_points, 13);
    }
}

mod read {
    // Sample input:
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    use nom::{
        bytes::complete::tag, character::complete as cc, combinator::all_consuming,
        multi::separated_list1, sequence::tuple, sequence::delimited, Finish, IResult,
    };
    use std::collections::HashSet;
    use super::Card;

    pub fn read_all_lines(i: &'static str) -> Vec<Card> {
        i.lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .collect()
    }

    // 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    fn parse_card(i: &str) -> IResult<&str, Card> {
        delimited(
            cc::multispace0,
            parse_lists,
            cc::multispace0
          )(i)
    }

    fn parse_lists(i: &str) -> IResult<&str, Card> {
        let (i, (winning, _, numbers)) = tuple((
            delimited(cc::multispace0, separated_list1(cc::multispace1, cc::u32), cc::multispace0), 
            tag("|"), 
            delimited(cc::multispace0, separated_list1(cc::multispace1, cc::u32), cc::multispace0)
        ))(i)?;
        Ok((i, Card { winning: HashSet::from_iter(winning.iter().cloned()), numbers }))
    }

    fn parse_line(i: &str) -> IResult<&str, Card> {
        let (i, _) = tuple((tag("Card"), cc::multispace1, cc::u32, tag(":")))(i)?;
        Ok(parse_card(i)?)
    }
}

