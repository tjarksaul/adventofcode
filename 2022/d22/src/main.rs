fn main() {
    let (puzzle, steps) = read::parse_all_lines(include_str!("../input.txt"));

    let result = find_end_position(&puzzle, &steps);

    dbg!(result);
}

#[derive(PartialEq, Debug)]
pub enum Step {
    Move(u64),
    Rotate(bool),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    row: u64,
    col: u64,
    facing: Direction,
}

impl Position {
    fn step(&self, puzzle: &Vec<Vec<Element>>) -> Self {
        let new_position = match self.facing {
            Direction::Up => {
                let mut row = self.row - 1;
                if row == 0 || Element::Empty == puzzle[(row - 1) as usize][self.col_index()] {
                    // we need to wrap around
                    row = (puzzle.len()
                        - puzzle
                            .iter()
                            .rev()
                            .position(|r| Element::Empty != r[self.col_index()])
                            .unwrap()) as u64;
                    // row += 1;
                }
                Self { row, ..*self }
            }
            Direction::Down => {
                let mut row = self.row + 1;
                if row as usize > puzzle.len()
                    || Element::Empty == puzzle[(row - 1) as usize][self.col_index()]
                {
                    // we need to wrap around
                    row = puzzle
                        .iter()
                        .position(|r| Element::Empty != r[self.col_index()])
                        .unwrap() as u64;
                    row += 1;
                }
                Self { row, ..*self }
            }
            Direction::Left => {
                let mut col = self.col - 1;
                if col == 0 || Element::Empty == puzzle[self.row_index()][(col - 1) as usize] {
                    // we need to wrap around
                    col = (puzzle[self.row_index()].len()
                        - puzzle[self.row_index()]
                            .iter()
                            .rev()
                            .position(|e| Element::Empty != *e)
                            .unwrap()) as u64;
                    // col += 1;
                }
                Self { col, ..*self }
            }
            Direction::Right => {
                let mut col = self.col + 1;
                if col as usize > puzzle[self.row_index()].len()
                    || Element::Empty == puzzle[self.row_index()][(col - 1) as usize]
                {
                    // we need to wrap around
                    col = (puzzle[self.row_index()]
                        .iter()
                        .position(|e| Element::Empty != *e)
                        .unwrap()) as u64;
                    col += 1;
                }
                Self { col, ..*self }
            }
        };

        if Element::Wall == puzzle[new_position.row_index()][new_position.col_index()] {
            *self
        } else {
            new_position
        }
    }

    fn value(&self) -> u64 {
        self.row * 1000 + self.col * 4 + self.facing.value()
    }

    fn row_index(&self) -> usize {
        (self.row - 1) as usize
    }

    fn col_index(&self) -> usize {
        (self.col - 1) as usize
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {
    fn value(&self) -> u64 {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }

    fn rotate(&self, clockwise: bool) -> Self {
        match self {
            Self::Right => {
                if clockwise {
                    Self::Down
                } else {
                    Self::Up
                }
            }
            Self::Down => {
                if clockwise {
                    Self::Left
                } else {
                    Self::Right
                }
            }
            Self::Left => {
                if clockwise {
                    Self::Up
                } else {
                    Self::Down
                }
            }
            Self::Up => {
                if clockwise {
                    Self::Right
                } else {
                    Self::Left
                }
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Element {
    Wall,
    Open,
    Empty,
}

impl Element {
    pub fn parse(i: &str) -> Self {
        match i {
            " " => Self::Empty,
            "." => Self::Open,
            "#" => Self::Wall,
            _ => panic!("how did we get here"),
        }
    }
}

fn find_end_position(input: &Vec<Vec<Element>>, steps: &Vec<Step>) -> u64 {
    let len = input.iter().map(|row| row.len()).max().unwrap();
    let mut puzzle = vec![];
    for i in 0..input.len() {
        let mut row = input[i].clone();
        if row.len() < len {
            let mut append = vec![Element::Empty; len - row.len()];
            row.append(&mut append);
        }
        puzzle.push(row);
    }
    let col = (puzzle[0]
        .iter()
        .position(|it| Element::Open == *it)
        .unwrap()
        + 1) as u64;
    let mut position = Position {
        row: 1,
        col,
        facing: Direction::Right,
    };

    for step in steps {
        match step {
            Step::Move(amount) => {
                for _ in 0..*amount {
                    let new_position = position.step(&puzzle);
                    if position == new_position {
                        // we moved into a wall, so we can stop here
                        break;
                    }
                    position = new_position;
                }
            }
            Step::Rotate(clockwise) => {
                position = Position {
                    facing: position.facing.rotate(*clockwise),
                    ..position
                };
            }
        }
    }

    position.value()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finds_end_position_correctly() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        let (puzzle, steps) = read::parse_all_lines(input);

        let position = find_end_position(&puzzle, &steps);

        assert_eq!(position, 6032);
    }
}

mod read {
    use super::{Element, Step};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete as cc,
        combinator::map,
        multi::{many1, separated_list1},
        sequence::tuple,
        IResult,
    };

    pub fn parse_all_lines(i: &str) -> (Vec<Vec<Element>>, Vec<Step>) {
        parse_all(i).unwrap().1
    }

    fn parse_all(i: &str) -> IResult<&str, (Vec<Vec<Element>>, Vec<Step>)> {
        let (i, (elements, _, steps)) = tuple((parse_all_elements, tag("\n\n"), parse_steps))(i)?;
        Ok((i, (elements, steps)))
    }

    fn parse_all_elements(i: &str) -> IResult<&str, Vec<Vec<Element>>> {
        separated_list1(tag("\n"), parse_elements)(i)
    }

    fn parse_elements(i: &str) -> IResult<&str, Vec<Element>> {
        many1(alt((
            map(tag(" "), Element::parse),
            map(tag("."), Element::parse),
            map(tag("#"), Element::parse),
        )))(i)
    }

    fn parse_steps(i: &str) -> IResult<&str, Vec<Step>> {
        many1(alt((
            map(tag("L"), |_| Step::Rotate(false)),
            map(tag("R"), |_| Step::Rotate(true)),
            map(cc::u64, Step::Move),
        )))(i)
    }
}
