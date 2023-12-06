fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let better_races = calculate_better_races(&input);

    dbg!(better_races);
}

fn calculate_better_races(input: &Vec<Race>) -> usize {
    input.iter().fold(1, |acc, cur| acc * calculate_better_ways(&cur))
}

fn calculate_better_ways(race: &Race) -> usize {
    let mut amount = 0;

    for i in 0..race.time {
        let speed = i;
        let distance = (race.time - i) * speed;
        if distance > race.max_dist {
            amount += 1
        }
    }
    amount
}

pub struct Race {
    time: usize,
    max_dist: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_better_races_correctly() {
        let input = vec![
            Race { time: 7, max_dist: 9, },
            Race { time: 15, max_dist: 40, },
            Race { time: 30, max_dist: 200, },
        ];

        let better_races = super::calculate_better_races(&input);

        assert_eq!(better_races, 288);
    }
}

mod read {
    // Sample input:
    // Time:      7  15   30
    // Distance:  9  40  200
    use nom::{
        bytes::complete::tag, character::complete as cc, combinator::all_consuming, combinator::map,
        multi::separated_list1, sequence::delimited, sequence::tuple, Finish, IResult,
    };
    use super::Race;

    pub fn read_all_lines(i: &'static str) -> Vec<Race> {
        all_consuming(parse_races)(i).finish().unwrap().1
    }

    fn parse_races(i: &str) -> IResult<&str, Vec<Race>> {
        let (i, times) = delimited(tuple((tag("Time:"), cc::multispace0)), separated_list1(cc::multispace1, parse_usize), cc::multispace0)(i)?;
        let (i, distances) = delimited(tuple((tag("Distance:"), cc::multispace0)), separated_list1(cc::multispace1, parse_usize), cc::multispace0)(i)?;

        if times.len() != distances.len() {
            panic!("Mismatched distances between times and distances");
        }

        let mut result = vec![];

        for i in 0..times.len() {
            result.push(Race { time: times[i], max_dist: distances[i] });
        }

        Ok((i, result))
    }

    fn parse_usize(i: &str) -> IResult<&str, usize> {
        map(cc::u32, |num: u32| num as usize)(i)
    }
}
