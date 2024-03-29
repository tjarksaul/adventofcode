use std::collections::HashSet;
use std::error::Error;
use std::{fmt, cmp};
use std::str::FromStr;
use pathfinding::prelude::bfs_reach;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2(&input);

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Input) -> usize {
    let nodes = find_visited_nodes(&input, (0, 0, Direction::Right));

    nodes.len()
}

fn part_2(input: &Input) -> usize {
    let mut starts = vec![];
    for i in 0..input.len() {
        starts.push((i, 0, Direction::Right));
        starts.push((i, input[i].len() - 1, Direction::Left));
    }
    for i in 0..input[0].len() {
        starts.push((0, i, Direction::Down));
    }
    let row = input.len() - 1;
    for i in 0..input[row].len() {
        starts.push((row, i, Direction::Up));
    }

    let mut max = 0;
    for start in starts {
        let nodes = find_visited_nodes(input, start);
        max = cmp::max(max, nodes.len());
    }

    max
}

fn find_visited_nodes(input: &Input, start: Node) -> HashSet<(usize, usize)> {
    let nodes = bfs_reach(start, |n| get_successors(n, &input));

    let mut set: HashSet<_> = nodes.map(|(y, x, _)| (y, x)).collect();
    set.insert((start.0, start.1));

    set
}

#[allow(dead_code)]
fn print_visited_nodes(input: &Input, nodes: &Vec<Node>) {
    for (i, row) in input.iter().enumerate() {
        for (j, chr) in row.iter().enumerate() {
            if let Some(nod) = nodes.iter().find(|nod| nod.0 == i && nod.1 == j) {
                if chr == &Type::Empty {
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

fn get_successors(node: &Node, input: &Input) -> Vec<Node> {
    let (y, x, dir) = *node;
    let typ = input[y][x];
    if typ == Type::Empty || (typ == Type::SplitterHorizontal && (dir == Direction::Left || dir == Direction::Right)) {
        match dir {
            Direction::Left => {
                if x == 0 { vec![] } else { vec![(y, x - 1, dir)] }
            }
            Direction::Right => {
                if x == input[y].len() - 1 { vec![] } else { vec![(y, x + 1, dir)] }
            }
            Direction::Down => {
                if y == input.len() - 1 { vec![] } else { vec![(y + 1, x, dir)] }
            }
            Direction::Up => {
                if y == 0 { vec![] } else { vec![(y - 1, x, dir)] }
            }
        }
    } else if typ == Type::MirrorLeft { // \
        match dir {
            Direction::Left => {
                if y == 0 { vec![] } else { vec![(y - 1, x, Direction::Up)] }
            }
            Direction::Right => {
                if y == input.len() - 1 { vec![] } else { vec![(y + 1, x, Direction::Down)] }
            }
            Direction::Down => {
                if x == input[y].len() - 1 { vec![] } else { vec![(y, x + 1, Direction::Right)] }
            }
            Direction::Up => {
                if x == 0 { vec![] } else { vec![(y, x - 1, Direction::Left)] }
            }
        }
    } else if typ == Type::MirrorRight { // /
        match dir {
            Direction::Left => {
                if y == input.len() - 1 { vec![] } else { vec![(y + 1, x, Direction::Down)] }
            }
            Direction::Right => {
                if y == 0 { vec![] } else { vec![(y - 1, x, Direction::Up)] }
            }
            Direction::Down => {
                if x == 0 { vec![] } else { vec![(y, x - 1, Direction::Left)] }
            }
            Direction::Up => {
                if x == input[y].len() - 1 { vec![] } else { vec![(y, x + 1, Direction::Right)] }
            }
        }
    } else if typ == Type::SplitterVertical {
        let mut res = vec![];
        if y != 0 {
            res.push((y - 1, x, Direction::Up));
        }
        if y != input.len() - 1 {
            res.push((y + 1, x, Direction::Down));
        }
        res
    } else if typ == Type::SplitterHorizontal {
        let mut res = vec![];
        if x != 0 {
            res.push((y, x - 1, Direction::Left));
        }
        if x != input[y].len() - 1 {
            res.push((y, x + 1, Direction::Right));
        }
        res
    } else {
        panic!("shouldnt");
    }
}

pub type Node = (usize, usize, Direction);
pub type Input = Vec<Vec<Type>>;

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub enum Type {
    Empty,
    MirrorLeft,
    MirrorRight,
    SplitterVertical,
    SplitterHorizontal,
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Type, ()> {
        match s {
            "." => Ok(Type::Empty),
            "\\" => Ok(Type::MirrorLeft),
            "/" => Ok(Type::MirrorRight),
            "|" => Ok(Type::SplitterVertical),
            "-" => Ok(Type::SplitterHorizontal),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Type::Empty => ".",
            Type::MirrorLeft => "\\",
            Type::MirrorRight => "/",
            Type::SplitterVertical => "|",
            Type::SplitterHorizontal => "-",
        };
        write!(f, "{out}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Input {
        let str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....".to_string();

        read::read_all_lines(str)
    }

    #[test]
    fn runs_part_1() {
        let input = get_input();

        let nodes = find_visited_nodes(&input, (0, 0, Direction::Right));
        assert_eq!(nodes.len(), 46);
    }

    #[test]
    fn runs_part_2() {
        let input = get_input();

        let best_case = part_2(&input);

        assert_eq!(best_case, 51);
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
