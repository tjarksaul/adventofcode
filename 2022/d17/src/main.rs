use std::cmp::max;
use std::collections::HashSet;
use std::fmt;

fn main() {
    let input = read::parse_input(include_str!("../input.txt"));

    let stack_height = calculate_stack_height(&input, 2022);

    dbg!(stack_height);
}

fn calculate_stack_height(wind_directions: &Vec<Direction>, rocks: u64) -> i64 {
    let mut stack_height = 0;
    let wind_directions_moves = wind_directions.len();
    let mut occupied_positons = HashSet::new();

    let mut moves = 0;
    for i in 0..rocks {
        println!("Rock {}", i + 1);

        let rock = Shape::at(i);
        let origin = Point(0, stack_height);

        let mut rock_points: Vec<_> = rock.points().iter().map(|point| *point + origin).collect();

        loop {
            // move by wind direction
            let direction = wind_directions[moves % wind_directions_moves].clone();
            moves += 1;
            let point_change = direction.point_change();

            let new_points: Vec<_> = rock_points
                .iter()
                .map(|&point| point + point_change)
                .collect();
            let is_out_of_bounds = new_points
                .iter()
                .position(|&point| point.0 < 0 || point.0 > 6 || occupied_positons.contains(&point))
                .is_some();
            if !is_out_of_bounds {
                rock_points = new_points;
            }

            // move one step down
            let new_points: Vec<_> = rock_points
                .iter()
                .map(|&point| point + Point(0, -1))
                .collect();
            let is_out_of_bounds = new_points
                .iter()
                .position(|&point| point.1 < 0 || occupied_positons.contains(&point))
                .is_some();
            if !is_out_of_bounds {
                rock_points = new_points;
            } else {
                occupied_positons.extend(rock_points.iter().map(|p| *p));
                stack_height = occupied_positons.iter().fold(0, |a, b| max(a, b.1)) + 1;
                break;
            }
        }
    }

    return stack_height;
}

#[allow(dead_code)]
fn draw_rocks(rocks: &HashSet<Point>) {
    let stack_height = rocks.iter().fold(0, |a, b| max(a, b.1));

    for y in (0..=stack_height).rev() {
        print!("|");
        let row_rocks: Vec<_> = rocks.iter().filter(|&point| point.1 == y).collect();
        for x in 0..7 {
            let has_point = row_rocks.iter().position(|&point| point.0 == x).is_some();
            if has_point {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
}

#[derive(
    PartialEq,
    Eq,
    Debug,
    Clone,
    Copy,
    Hash,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
)]
pub struct Point(i64, i64);

pub enum Shape {
    HorizontalLine,
    Plus,
    BackwardsL,
    VerticalLine,
    Square,
}

impl Shape {
    fn at(index: u64) -> Shape {
        let index = index % 5;
        match index {
            0 => Shape::HorizontalLine,
            1 => Shape::Plus,
            2 => Shape::BackwardsL,
            3 => Shape::VerticalLine,
            4 => Shape::Square,
            _ => panic!("how did this happen?"),
        }
    }

    fn points(&self) -> Vec<Point> {
        match self {
            Shape::HorizontalLine => vec![Point(2, 3), Point(3, 3), Point(4, 3), Point(5, 3)],
            Shape::Plus => vec![
                Point(2, 4),
                Point(3, 5),
                Point(3, 4),
                Point(3, 3),
                Point(4, 4),
            ],
            Shape::BackwardsL => vec![
                Point(2, 3),
                Point(3, 3),
                Point(4, 3),
                Point(4, 4),
                Point(4, 5),
            ],
            Shape::VerticalLine => vec![Point(2, 3), Point(2, 4), Point(2, 5), Point(2, 6)],
            Shape::Square => vec![Point(2, 3), Point(2, 4), Point(3, 3), Point(3, 4)],
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Shape::HorizontalLine => "####",
            Shape::Plus => ".#.\n###\n.#.",
            Shape::BackwardsL => "..#\n..#\n###",
            Shape::VerticalLine => "#\n#\n#\n#",
            Shape::Square => "##\n##",
        };
        write!(f, "{out}")
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Direction::Left => "<",
            Direction::Right => ">",
        };
        write!(f, "{out}")
    }
}

type Err = ();

impl Direction {
    pub fn parse(input: char) -> Result<Direction, Err> {
        match input {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(()),
        }
    }

    fn point_change(&self) -> Point {
        match self {
            Direction::Left => Point(-1, 0),
            Direction::Right => Point(1, 0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counts_fallen_rocks_correctly() {
        let pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let input = read::parse_input(&pattern);

        let stack_height = calculate_stack_height(&input, 2022);

        assert_eq!(stack_height, 3068);
    }
}

mod read {
    use super::Direction;

    pub fn parse_input(input: &str) -> Vec<Direction> {
        let chars = input.chars();
        chars.map(|c| Direction::parse(c).unwrap()).collect()
    }
}
