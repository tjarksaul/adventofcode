fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let possible_game_sum = find_possible_game_sum(&input, 12, 13, 14);

    dbg!(possible_game_sum);
}

fn find_possible_game_sum(input: &Vec<Vec<Cubes>>, num_red: usize, num_green: usize, num_blue: usize) -> usize {
    let mut game_sum = 0;
    for (game_nr, game) in input.iter().enumerate() {
        if is_game_possible(&game, num_red, num_green, num_blue) {
            game_sum += game_nr + 1;
        }
    }
    game_sum
}

fn is_game_possible(game: &Vec<Cubes>, num_red: usize, num_green: usize, num_blue: usize) -> bool {
    for cube in game.iter() {
        if cube.red > num_red || cube.green > num_green || cube.blue > num_blue {
            return false;
        }
    }
    return true;
}

pub struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_possible_game_sum_correctly() {
        let input = vec![
            vec![Cubes { red: 4, green: 0, blue: 3 }, Cubes { red: 1, green: 2, blue: 6 }, Cubes { red: 0, green: 2, blue: 0 }],
            vec![Cubes { blue: 1, green: 2, red: 0 }, Cubes { green: 3, blue: 4, red: 1 }, Cubes {green: 1, blue: 1, red: 0 }],
            vec![Cubes { red: 20, green: 8, blue: 6 }, Cubes { red: 4, green: 13, blue: 5 }, Cubes { red: 1, green: 5, blue: 0 }],
            vec![Cubes { red: 3, green: 1, blue: 6 }, Cubes { red: 6, green: 3, blue: 0 }, Cubes { red: 14, green: 3, blue: 15 }],
            vec![Cubes { red: 6, green: 3, blue: 1 }, Cubes { red: 1, green: 2, blue: 2 }],
        ];

        let possible_game_sum = super::find_possible_game_sum(&input, 12, 13, 14);

        assert_eq!(possible_game_sum, 8);
    }
}

mod read {
    // Sample input:
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    use nom::{
        bytes::complete::tag, character::complete as cc, combinator::all_consuming,
        branch::alt, multi::separated_list1, sequence::tuple, Finish, IResult,
    };
    use super::Cubes;

    pub fn read_all_lines(i: &'static str) -> Vec<Vec<Cubes>> {
        i.lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .collect()
    }

    // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    fn parse_cubes(i: &str) -> IResult<&str, Cubes> {
        let (i, cubes) = separated_list1(tag(","), parse_item)(i)?;
        Ok((i, Cubes {
            red: cubes.iter().find(|v| v.0 == "red").map(|v| v.1).unwrap_or(0),
            green: cubes.iter().find(|v| v.0 == "green").map(|v| v.1).unwrap_or(0),
            blue: cubes.iter().find(|v| v.0 == "blue").map(|v| v.1).unwrap_or(0),
        }))
    }

    fn parse_item(i: &str) -> IResult<&str, (&str, usize)> {
        let (i, (_, val, _, color)) = tuple((tag(" "), cc::u32, tag(" "), alt((tag("red"), tag("green"), tag("blue")))))(i)?;
        Ok((i, (color, val as usize)))
    }

    fn parse_line(i: &str) -> IResult<&str, Vec<Cubes>> {
        let (i, (_, _, _)) = tuple((tag("Game "), cc::u32, tag(":")))(i)?;
        separated_list1(tag(";"), parse_cubes)(i)
    }
}

