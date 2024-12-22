use derive_more::Display;
use std::{collections::HashSet, error::Error, fmt};

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let mut part_1_input = input.clone();
    let part1 = part_1(&mut part_1_input);

    let mut part_2_input = input.clone();
    let part2 = part_2(&mut part_2_input);

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

fn part_2(input: &mut Map) -> usize {
    let mut loops = 0;

    // let's just force us through and try all possible positions for new obstacles
    for i in 0..input.rows {
        for j in 0..input.cols {
            let obstacle_pos = Position(i, j);

            // we can't put a new obstacle at the guard's start position
            // and we skip positions where there's already an obstacle
            if obstacle_pos == input.guard.pos || input.obstacles.contains(&obstacle_pos) {
                continue;
            }

            // clone the map so that we don't fill it with obstacles
            let mut copy = input.clone();
            copy.obstacles.push(obstacle_pos);
            let mut visited = HashSet::from([input.guard]);

            loop {
                let pos = copy.guard.pos;
                let rot = copy.guard.orientation.clone();

                match copy.mov() {
                    Ok(new_pos) => {
                        // if we moved or rotated we check if we are in the same place as before
                        if (new_pos != pos || copy.guard.orientation != rot)
                            && visited.contains(&copy.guard)
                        {
                            println!("Found a loop with new obstacle at {}", obstacle_pos);
                            loops += 1;
                            break;
                        }

                        visited.insert(copy.guard.clone());
                    }
                    Err(_) => {
                        // we exited the map
                        break;
                    }
                }
            }
        }
    }

    loops
}

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

impl Clone for Map {
    fn clone(&self) -> Self {
        Map {
            guard: GuardPosition {
                pos: self.guard.pos.clone(),
                orientation: self.guard.orientation.clone(),
            },
            obstacles: self.obstacles.clone(),
            rows: self.rows.clone(),
            cols: self.cols.clone(),
        }
    }
}

#[derive(Debug, Display)]
struct OutOfBounds;

impl Error for OutOfBounds {}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display)]
#[display("{}, {}", _0, _1)]
// (y, x)
struct Position(usize, usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

    fn get_input() -> Map {
        Map {
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
        }
    }

    #[test]
    fn runs_part_1() {
        let mut map = get_input();

        let result = part_1(&mut map);

        assert_eq!(41, result);
    }

    #[test]
    fn runs_part_2() {
        let mut map = get_input();

        let result = part_2(&mut map);

        assert_eq!(6, result);
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
