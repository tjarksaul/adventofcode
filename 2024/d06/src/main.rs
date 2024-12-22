use derive_more::Display;
use std::{collections::HashSet, error::Error, fmt};

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let mut part_1_input = input.clone();

    let part1 = part_1(&mut part_1_input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &mut Map) -> usize {
    let verbose = false;
    let mut visited = HashSet::from([input.guard.pos]);

    if verbose {
        input.print(&visited);
    }

    loop {
        match input.mov() {
            Ok(new_pos) => {
                visited.insert(new_pos);
            }
            Err(_) => {
                break;
            }
        }
    }

    if verbose {
        input.print(&visited);
    }

    visited.len()
}

fn part_2() -> usize {
    0
}

#[derive(Clone)]
struct Map {
    guard: GuardPosition,
    obstacles: Vec<Position>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn mov(&mut self) -> Result<Position, Box<dyn Error>> {
        let new_pos = match self.guard.orientation {
            Orientation::Right => {
                if self.guard.pos.1 == self.cols - 1 {
                    return Err(Box::new(OutOfBounds));
                }
                Position(self.guard.pos.0, self.guard.pos.1 + 1)
            }
            Orientation::Down => {
                if self.guard.pos.0 == self.rows - 1 {
                    return Err(Box::new(OutOfBounds));
                }
                Position(self.guard.pos.0 + 1, self.guard.pos.1)
            }
            Orientation::Left => {
                if self.guard.pos.1 == 0 {
                    return Err(Box::new(OutOfBounds));
                }
                Position(self.guard.pos.0, self.guard.pos.1 - 1)
            }
            Orientation::Up => {
                if self.guard.pos.0 == 0 {
                    return Err(Box::new(OutOfBounds));
                }
                Position(self.guard.pos.0 - 1, self.guard.pos.1)
            }
        };

        if !self.obstacles.contains(&new_pos) {
            self.guard.pos = new_pos;
            return Ok(new_pos);
        } else {
            self.guard.rotate();
            return Ok(self.guard.pos);
        }
    }
}

impl Map {
    fn print(&self, visited: &HashSet<Position>) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let pos = Position(i, j);
                if self.guard.pos == pos {
                    print!("{}", self.guard.orientation);
                } else if self.obstacles.contains(&pos) {
                    print!("#");
                } else if visited.contains(&pos) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        println!("");
    }
}

#[derive(Debug, Display)]
struct OutOfBounds;

impl Error for OutOfBounds {}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display)]
#[display("{}, {}", _0, _1)]
// (y, x)
struct Position(usize, usize);

#[derive(Clone)]
enum Orientation {
    Right,
    Down,
    Left,
    Up,
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Right => write!(f, ">"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Up => write!(f, "^"),
        }
    }
}

#[derive(Clone)]
struct GuardPosition {
    pos: Position,
    orientation: Orientation,
}

impl GuardPosition {
    fn rotate(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Up => Orientation::Right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let mut map = Map {
            guard: GuardPosition {
                pos: Position(6, 4),
                orientation: Orientation::Up,
            },
            obstacles: vec![
                Position(0, 4),
                Position(1, 9),
                Position(3, 2),
                Position(4, 7),
                Position(6, 1),
                Position(7, 8),
                Position(8, 0),
                Position(9, 6),
            ],
            rows: 10,
            cols: 10,
        };

        let result = part_1(&mut map);

        assert_eq!(41, result);
    }
}

mod read {
    use super::{GuardPosition, Map, Orientation, Position};

    pub fn read_all_lines(i: String) -> Map {
        // guard: GuardPosition,
        // obstacles: Vec<Position>,
        // rows: usize,
        // cols: usize,
        let lines: Vec<_> = i.lines().collect();
        let rows = lines.len();
        let mut cols = 0;
        let mut obstacles = vec![];
        let mut guard_pos = Position(0, 0);

        for r in 0..lines.len() {
            let chars: Vec<char> = lines[r].to_owned().chars().collect();
            cols = chars.len();
            for c in 0..lines[r].len() {
                if chars[c] == '^' {
                    guard_pos = Position(r, c);
                } else if chars[c] == '#' {
                    obstacles.push(Position(r, c));
                }
            }
        }

        Map {
            guard: GuardPosition {
                pos: guard_pos,
                orientation: Orientation::Up,
            },
            obstacles,
            rows,
            cols,
        }
    }
}
