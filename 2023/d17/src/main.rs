use std::error::Error;

use pathfinding::prelude::astar;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Input) -> usize {
    let goal = Node(input.len() - 1, input[input.len() - 1].len() - 1, Direction::Up, 0);
    let path = astar(&Node(0, 0, Direction::Right, 0), |p| p.get_successors(&input), |p| p.distance(&goal) / 3,
                   |Node(y, x, _, _)| *y == goal.0 && *x == goal.1).unwrap();

    path.1
}

fn part_2() -> usize {
    0
}

#[allow(dead_code)]
fn print_visited_nodes(input: &Input, nodes: &Vec<Node>) {
    for (i, row) in input.iter().enumerate() {
        for (j, chr) in row.iter().enumerate() {
            if i != 0 || j != 0 {
                if let Some(nod) = nodes.iter().find(|nod| nod.0 == i && nod.1 == j) {
                    match nod.2 {
                        Direction::Down => print!("v"),
                        Direction::Up => print!("^"),
                        Direction::Left => print!("<"),
                        Direction::Right => print!(">"),
                    };
                    continue;
                } 
            }
                print!("{chr}");
        }
        print!("\n");
    }
}

pub type Input = Vec<Vec<usize>>;

#[derive(Eq, Clone, Copy, Debug, Hash)]
pub struct Node(usize, usize, Direction, usize);

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Node {
    fn distance(&self, other: &Node) -> usize {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as usize
    }

    fn get_successors(&self, input: &Input) -> Vec<(Node, usize)> {
        let Node(y, x, dir, steps) = *self;
        let mut res = vec![];
        // going straight
        if steps < 3 {
            if y != 0 && dir == Direction::Up {
                res.push((Node(y - 1, x, dir, steps + 1), input[y - 1][x]));
            } else if y != input.len() - 1 && dir == Direction::Down {
                res.push((Node(y + 1, x, dir, steps + 1), input[y + 1][x]));
            } else if x != 0 && dir == Direction::Left {
                res.push((Node(y, x - 1, dir, steps + 1), input[y][x - 1]));
            } else if x != input[y].len() - 1 && dir == Direction::Right {
                res.push((Node(y, x + 1, dir, steps + 1), input[y][x + 1]))
            }
        }

        // turning left
        if dir == Direction::Left && y != input.len() - 1 {
            res.push((Node(y + 1, x, Direction::Down, 1), input[y + 1][x]));
        }
        if dir == Direction::Right && y != 0 {
            res.push((Node(y - 1, x, Direction::Up, 1), input[y - 1][x]));

        }
        if dir == Direction::Down && x != input[y].len() - 1 {
            res.push((Node(y, x + 1, Direction::Right, 1), input[y][x + 1]));
        }
        if dir == Direction::Up && x != 0 {
            res.push((Node(y, x - 1, Direction::Left, 1), input[y][x - 1]));
        }

        // turning right
        if dir == Direction::Left && y != 0 {
            res.push((Node(y - 1, x, Direction::Up, 1), input[y - 1][x]));
        }
        if dir == Direction::Right && y != input.len() - 1 {
            res.push((Node(y + 1, x, Direction::Down, 1), input[y + 1][x]));
        }
        if dir == Direction::Down && x != 0 {
            res.push((Node(y, x - 1, Direction::Left, 1), input[y][x - 1]));
        }
        if dir == Direction::Up && x != input[y].len() - 1 {
            res.push((Node(y, x + 1, Direction::Right, 1), input[y][x + 1]));
        }

        // println!("Successors for {self:?}: {res:?}");

        res
    }
    
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::read::read_all_lines;

    use super::*;

    #[test]
    fn runs_part_1() {
        let str = read_to_string("test.txt").unwrap();
        let input = read_all_lines(str);

        let path = part_1(&input);

        assert_eq!(path, 102);
    }
}

mod read {
    use super::Input;

    pub fn read_all_lines(i: String) -> Input {
        i.lines()
            .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect()
    }
}
