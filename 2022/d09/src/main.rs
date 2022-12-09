use std::fmt;
use std::str::FromStr;

fn main() {
    let input = read::read_input(String::from("input.txt"));

    let tail_visited_nodes_count = count_tail_visited_nodes(&input);

    println!("Found {tail_visited_nodes_count} visited nodes.");
}

const GRID_WIDTH: usize = 1000;
const START_POSITION: usize = 500;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        };
        write!(f, "{out}")
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Move {
    fn print(&self) {
        println!("== {} {} ==", self.direction, self.steps)
    }
}

pub struct Move {
    direction: Direction,
    steps: usize,
}

fn count_tail_visited_nodes(moves: &Vec<Move>) -> usize {
    let mut head_position = (START_POSITION, START_POSITION);
    let mut tail_position = (START_POSITION, START_POSITION);
    let mut visited = vec![vec![false; GRID_WIDTH]; GRID_WIDTH];
    visited[START_POSITION][START_POSITION] = true;

    println!("== Initial state ==\n");

    // print_grid(&visited, head_position, tail_position);

    for mov in moves {
        println!("");
        mov.print();
        println!("");
        for _ in 0..mov.steps {
            if Direction::Up == mov.direction {
                head_position = (head_position.0 + 1, head_position.1);
            } else if Direction::Down == mov.direction {
                head_position = (head_position.0 - 1, head_position.1);
            } else if Direction::Right == mov.direction {
                head_position = (head_position.0, head_position.1 + 1);
            } else {
                head_position = (head_position.0, head_position.1 - 1);
            }

            if (head_position.0 as i64 - tail_position.0 as i64).abs() >= 2
                || (head_position.1 as i64 - tail_position.1 as i64).abs() >= 2
            {
                let mut x = tail_position.1;
                let mut y = tail_position.0;
                if head_position.1 != tail_position.1 {
                    x = if head_position.1 > tail_position.1 {
                        tail_position.1 + 1
                    } else {
                        tail_position.1 - 1
                    };
                }
                if head_position.0 != tail_position.0 {
                    y = if head_position.0 > tail_position.0 {
                        tail_position.0 + 1
                    } else {
                        tail_position.0 - 1
                    };
                }
                tail_position = (y, x);
                visited[tail_position.0][tail_position.1] = true;
            }

            // print_grid(&visited, head_position, tail_position);
        }
    }

    return visited
        .iter()
        .flat_map(|a| a)
        .fold(0, |a, b| a + *b as usize);
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<bool>>, head_position: (usize, usize), tail_position: (usize, usize)) {
    for y in (0..grid.len()).rev() {
        for x in 0..grid[y].len() {
            let mut chr = ".";
            if (y, x) == head_position {
                chr = "H";
            } else if (y, x) == tail_position {
                chr = "T";
            } else if (y, x) == (START_POSITION, START_POSITION) {
                chr = "s";
            } else if grid[y][x] {
                chr = "#";
            }
            print!("{chr}");
        }
        println!("");
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Move> {
        return vec![
            Move {
                direction: Direction::Right,
                steps: 4,
            },
            Move {
                direction: Direction::Up,
                steps: 4,
            },
            Move {
                direction: Direction::Left,
                steps: 3,
            },
            Move {
                direction: Direction::Down,
                steps: 1,
            },
            Move {
                direction: Direction::Right,
                steps: 4,
            },
            Move {
                direction: Direction::Down,
                steps: 1,
            },
            Move {
                direction: Direction::Left,
                steps: 5,
            },
            Move {
                direction: Direction::Right,
                steps: 2,
            },
        ];
    }

    #[test]
    fn test_counts_correctly() {
        let moves = get_input();

        let tail_visited_nodes = count_tail_visited_nodes(&moves);

        assert_eq!(tail_visited_nodes, 13);
    }
}

mod read {
    use std::fs;
    use std::str::FromStr;

    use super::Direction;
    use super::Move;

    pub fn read_input(fname: String) -> Vec<Move> {
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

        let lines: Vec<&str> = contents.lines().collect();

        return lines
            .iter()
            .map(|line| {
                let mut splits = line.split(' ');
                let direction = splits.next().unwrap();
                let steps = splits.next().unwrap();

                return Move {
                    direction: Direction::from_str(direction).unwrap(),
                    steps: steps.parse().unwrap(),
                };
            })
            .collect();
    }
}
