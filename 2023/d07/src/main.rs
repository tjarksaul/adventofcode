use std::collections::HashMap;
use std::cmp::Ordering;
use std::str::FromStr;
use std::fmt::Formatter;

fn main() {
    let mut input = read::read_all_lines(include_str!("../input.txt"));

    let winning_amount = calculate_winning_amount(&mut input);

    dbg!(winning_amount);
}

fn calculate_winning_amount(input: &mut Vec<Hand>) -> usize {
    input.sort();
    
    let mut result = 0;
    for i in 0..input.len() {
        result += input[i].bid * (i + 1);
    }
    result
}

#[derive(PartialEq, Eq, Ord)]
#[derive(Debug)]
pub struct Hand {
    cards: (Card, Card, Card, Card, Card),
    bid: usize,
}

impl Hand {
    fn get_type(&self) -> Type {
        let mut amounts = HashMap::<&Card, usize>::new();
        for card in [&self.cards.0, &self.cards.1, &self.cards.2, &self.cards.3, &self.cards.4] {
            let amount = amounts.get(card).unwrap_or(&0);
            amounts.insert(card, *amount + 1);
        }
        // dbg!(amounts);
        let mut max_amount = 1;
        let mut max_card = &Card::Two;
        for (card, amount) in &amounts {
            if *amount == 5 {
                return Type::Five;
            }
            if max_amount < *amount {
                max_amount = *amount;
                max_card = *card;
            }
        }
        if max_amount == 4 {
            return Type::Four;
        }
        if max_amount == 3 {
            for (_, amount) in &amounts {
                if *amount == 2 {
                    return Type::FullHouse;
                }
            }
            return Type::Three;
        }
        if max_amount == 2 {
            for (card, amount) in &amounts {
                if *amount == 2 && card != &max_card {
                    return Type::TwoPair;
                }
            }
            return Type::Pair;
        }
        Type::High
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_type = self.get_type();
        let other_type = other.get_type();
        if self_type > other_type {
            return Some(Ordering::Greater);
        }
        if other_type > self_type {
            return Some(Ordering::Less);
        }
        for (self_card, other_card) in [
            (&self.cards.0, &other.cards.0), 
            (&self.cards.1, &other.cards.1), 
            (&self.cards.2, &other.cards.2), 
            (&self.cards.3, &other.cards.3), 
            (&self.cards.4, &other.cards.4),
        ] {
            if self_card > other_card {
                return Some(Ordering::Greater);
            }
            if other_card > self_card {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "{}{}{}{}{} {}", self.cards.0 as u8 as char, self.cards.1 as u8 as char, self.cards.2 as u8 as char, self.cards.3 as u8 as char, self.cards.4 as u8 as char, self.bid)
    }
}


#[repr(u8)]
#[derive(PartialEq, Eq, Ord)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(Clone, Copy)]
pub enum Card {
    Ace = b'A',
    King = b'K',
    Queen = b'Q',
    Jack = b'J',
    Ten = b'T',
    Nine = b'9',
    Eight = b'8',
    Seven = b'7',
    Six = b'6',
    Five = b'5',
    Four = b'4',
    Three = b'3',
    Two = b'2',
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return match self {
            Self::Ace => match other {
                Self::Ace => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::King => match other {
                Self::Ace => Some(Ordering::Less),
                Self::King => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::Queen => match other {
                Self::Ace => Some(Ordering::Less),
                Self::King => Some(Ordering::Less),
                Self::Queen => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::Jack => match other {
                Self::Ace => Some(Ordering::Less),
                Self::King => Some(Ordering::Less),
                Self::Queen => Some(Ordering::Less),
                Self::Jack => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::Ten => match other {
                Self::Ace => Some(Ordering::Less),
                Self::King => Some(Ordering::Less),
                Self::Queen => Some(Ordering::Less),
                Self::Jack => Some(Ordering::Less),
                Self::Ten => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::Nine => match other {
                Self::Ace => Some(Ordering::Less),
                Self::King => Some(Ordering::Less),
                Self::Queen => Some(Ordering::Less),
                Self::Jack => Some(Ordering::Less),
                Self::Ten => Some(Ordering::Less),
                Self::Nine => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::Eight => match other {
                Self::Ace => Some(Ordering::Less),
                Self::King => Some(Ordering::Less),
                Self::Queen => Some(Ordering::Less),
                Self::Jack => Some(Ordering::Less),
                Self::Ten => Some(Ordering::Less),
                Self::Nine => Some(Ordering::Less),
                Self::Eight => Some(Ordering::Equal),
                Self::Seven => Some(Ordering::Greater),
                Self::Six => Some(Ordering::Greater),
                Self::Five => Some(Ordering::Greater),
                Self::Four => Some(Ordering::Greater),
                Self::Three => Some(Ordering::Greater),
                Self::Two => Some(Ordering::Greater),
            },
            Self::Seven => match other {
                Self::Seven => Some(Ordering::Equal),
                Self::Six => Some(Ordering::Greater),
                Self::Five => Some(Ordering::Greater),
                Self::Four => Some(Ordering::Greater),
                Self::Three => Some(Ordering::Greater),
                Self::Two => Some(Ordering::Greater),
                _ => Some(Ordering::Less),
            },
            Self::Six => match other {
                Self::Six => Some(Ordering::Equal),
                Self::Five => Some(Ordering::Greater),
                Self::Four => Some(Ordering::Greater),
                Self::Three => Some(Ordering::Greater),
                Self::Two => Some(Ordering::Greater),
                _ => Some(Ordering::Less),
            },
            Self::Five => match other {
                Self::Five => Some(Ordering::Equal),
                Self::Four => Some(Ordering::Greater),
                Self::Three => Some(Ordering::Greater),
                Self::Two => Some(Ordering::Greater),
                _ => Some(Ordering::Less),
            },
            Self::Four => match other {
                Self::Four => Some(Ordering::Equal),
                Self::Three => Some(Ordering::Greater),
                Self::Two => Some(Ordering::Greater),
                _ => Some(Ordering::Less),
            },
            Self::Three => match other {
                Self::Three => Some(Ordering::Equal),
                Self::Two => Some(Ordering::Greater),
                _ => Some(Ordering::Less),
            },
            Self::Two => match other {
                Self::Two => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
            },
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Card, ()> {
        match s {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Ten),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Ord)]
#[derive(Debug)]
pub enum Type {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    High,
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return match self {
            Self::Five => match other {
                Self::Five => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::Four => match other {
                Self::Five => Some(Ordering::Less),
                Self::Four => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::FullHouse => match other {
                Self::Five => Some(Ordering::Less),
                Self::Four => Some(Ordering::Less),
                Self::FullHouse => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::Three => match other {
                Self::Five => Some(Ordering::Less),
                Self::Four => Some(Ordering::Less),
                Self::FullHouse => Some(Ordering::Less),
                Self::Three => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::TwoPair => match other {
                Self::Pair => Some(Ordering::Greater),
                Self::High => Some(Ordering::Greater),
                Self::TwoPair => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
            },
            Self::Pair => match other {
                Self::High => Some(Ordering::Greater),
                Self::Pair => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
            },
            Self::High => match other {
                Self::High => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Hand> {
        // 32T3K 765
        // T55J5 684
        // KK677 28
        // KTJJT 220
        // QQQJA 483
        vec![
            Hand { cards: (Card::Three, Card::Two, Card::Ten, Card::Three, Card::King), bid: 765 },
            Hand { cards: (Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five), bid: 684 },
            Hand { cards: (Card::King, Card::King, Card::Six, Card::Seven, Card::Seven), bid: 28 },
            Hand { cards: (Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten), bid: 220 },
            Hand { cards: (Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace), bid: 483 },
        ]
    }

    #[test]
    fn test_calculates_winning_amount_correctly() {
        let mut input = get_input();

        let winning_amount = super::calculate_winning_amount(&mut input);

        assert_eq!(winning_amount, 6440);
    }

    #[test]
    fn test_calculates_winning_amount2_correctly() {
        let mut input = vec![
            Hand {
                cards: (
                    Card::King,
                    Card::King,
                    Card::King,
                    Card::King,
                    Card::Ace,
                ),
                bid: 513,
            },
            Hand {
                cards: (
                    Card::Four,
                    Card::Ace,
                    Card::Four,
                    Card::Four,
                    Card::Four,
                ),
                bid: 635,
            },        
        ];

        let winning_amount = super::calculate_winning_amount(&mut input);

        assert_eq!(winning_amount, 1661);
    }

    #[test]
    fn test_finds_type_correctly() {
        let input = get_input();
        let types = vec![
            Type::Pair,
            Type::Three,
            Type::TwoPair,
            Type::TwoPair,
            Type::Three,
        ];

        for i in 0..input.len() {
            let typ = input[i].get_type();

            assert_eq!(typ, types[i]);
        }
    }
}
mod read {
    // Sample input:
    // 32T3K 765
    // T55J5 684
    use nom::{
        character::complete as cc, combinator::all_consuming, combinator::map,
        sequence::tuple, Finish, IResult,
    };
    use super::{Card, Hand};

    pub fn read_all_lines(i: &'static str) -> Vec<Hand> {
        i.lines()
            .map(|l| all_consuming(parse_hand)(l).finish().unwrap().1)
            .collect()
    }

    fn parse_hand(i: &str) -> IResult<&str, Hand> {
        let (i, (cards, _, bid)) = tuple((parse_cards, cc::multispace1, parse_usize))(i)?;

        Ok((i, Hand { cards, bid }))
    }

    fn parse_cards(i: &str) -> IResult<&str, (Card, Card, Card, Card, Card)> {
        let (i, cards) = tuple((parse_card, parse_card, parse_card, parse_card, parse_card))(i)?;

        Ok((i, cards))
    }

    fn parse_card(i: &str) -> IResult<&str, Card> {
        let (i, char) = cc::one_of("AKQJT98765432")(i)?;

        Ok((i, char.to_string().parse::<Card>().unwrap()))
    }

    fn parse_usize(i: &str) -> IResult<&str, usize> {
        map(cc::u32, |num: u32| num as usize)(i)
    }
}
