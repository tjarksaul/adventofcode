use std::fmt;
use std::str::FromStr;

fn main() {
    let input = read::read_input(String::from("input.txt"));

    let tail_visited_nodes_count = count_tail_visited_nodes(&input);
    let tail_visited_nodes_2_count = count_tail_visited_nodes_2(&input);

    println!("Found {tail_visited_nodes_count} visited nodes.");
    println!("Found {tail_visited_nodes_2_count} visited nodes for part 2.");
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

fn count_tail_visited_nodes_2(moves: &Vec<Move>) -> usize {
    let mut knots = vec![
        (START_POSITION, START_POSITION),
        (START_POSITION, START_POSITION),
        (START_POSITION, START_POSITION),
        (START_POSITION, START_POSITION),
        (START_POSITION, START_POSITION),
        (START_POSITION, START_POSITION),
        (START_POSITION, START_POSITION),
        (START_POSITION, START_POSITION),
        (START_POSITION, START_POSITION),
        (START_POSITION, START_POSITION),
    ];
    let mut visited = vec![vec![false; GRID_WIDTH]; GRID_WIDTH];
    visited[START_POSITION][START_POSITION] = true;

    println!("== Initial state ==\n");

    // print_grid(&visited, &knots);

    for mov in moves {
        println!("");
        mov.print();
        println!("");
        for _ in 0..mov.steps {
            if Direction::Up == mov.direction {
                knots[0] = (knots[0].0 + 1, knots[0].1);
            } else if Direction::Down == mov.direction {
                knots[0] = (knots[0].0 - 1, knots[0].1);
            } else if Direction::Right == mov.direction {
                knots[0] = (knots[0].0, knots[0].1 + 1);
            } else {
                knots[0] = (knots[0].0, knots[0].1 - 1);
            }

            for i in 1..knots.len() {
                let prev_knot = knots[i - 1];
                let curr_knot = knots[i];
                if (prev_knot.0 as i64 - curr_knot.0 as i64).abs() >= 2
                    || (prev_knot.1 as i64 - curr_knot.1 as i64).abs() >= 2
                {
                    let mut x = curr_knot.1;
                    let mut y = curr_knot.0;
                    if prev_knot.1 != curr_knot.1 {
                        x = if prev_knot.1 > curr_knot.1 {
                            curr_knot.1 + 1
                        } else {
                            curr_knot.1 - 1
                        };
                    }
                    if prev_knot.0 != curr_knot.0 {
                        y = if prev_knot.0 > curr_knot.0 {
                            curr_knot.0 + 1
                        } else {
                            curr_knot.0 - 1
                        };
                    }
                    knots[i] = (y, x);
                }
            }

            visited[knots[9].0][knots[9].1] = true;

            // print_grid(&visited, &knots);
        }
    }

    return visited
        .iter()
        .flat_map(|a| a)
        .fold(0, |a, b| a + *b as usize);
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<bool>>, knot_positions: &Vec<(usize, usize)>) {
    for y in (0..grid.len()).rev() {
        for x in 0..grid[y].len() {
            let mut chr = ".";
            let mut print_knot: usize = 0;
            if (y, x) == knot_positions[0] {
                chr = "H";
            } else {
                for i in 1..knot_positions.len() {
                    if (y, x) == knot_positions[i] {
                        print_knot = i;
                        break;
                    }
                }
            }

            if chr == "." {
                if (y, x) == (START_POSITION, START_POSITION) {
                    chr = "s";
                } else if grid[y][x] {
                    chr = "#";
                }
            }
            if print_knot > 0 {
                print!("{print_knot}");
            } else {
                print!("{chr}");
            }
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

    #[test]
    fn test_counts_long_correctly() {
        let moves = get_input();

        let tail_visited_nodes = count_tail_visited_nodes_2(&moves);

        assert_eq!(tail_visited_nodes, 1);
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
