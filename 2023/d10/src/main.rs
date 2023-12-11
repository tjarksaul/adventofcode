use std::fmt::Formatter;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut input = read::read_all_lines(include_str!("../input.txt"));

    let (full_pipe, pipe_network) = find_full_pipe(&mut input);

    let network = clean_network(&input, &pipe_network);
    let inside_node_count = find_inside_nodes(&network);

    dbg!(full_pipe, inside_node_count);
}

fn find_full_pipe(input: &mut Network) -> (usize, Vec<Pos>) {
    let start = find_start(&input);

    for (pos, dir) in [
        (Pos(start.0 + 1, start.1 + 0), "D"), 
        (Pos(start.0 - 1, start.1 + 0), "U"), 
        (Pos(start.0 + 0, start.1 - 1), "L"), 
        (Pos(start.0 + 0, start.1 + 1), "R"),
    ] {
    let mut full_pipe = vec![];
    let mut previous = start;
        let mut position: Option<Pos> = Some(pos);
        let mut steps = 1;
        full_pipe.push(pos);

        while position.is_some() {
            let pos = position.unwrap();
            if let Some(pipe) = input[pos.0][pos.1] {
                let pipe = pipe.0;
                let next = pos.next(&pipe, &previous);
                if let Some(cur) = next {
                    full_pipe.push(cur);
                    steps += 1;
                    if cur == start {
                        let starting_pipe = interpolate_starting_pipe(dir, &start, &pos);
                        input[start.0][start.1] = Some(Pipe(starting_pipe, start));
                        return (steps / 2, full_pipe);
                    }
                }
                previous = pos;
                position = next;
            } else { break; }
        }
    }
    
    panic!("Nothing found");
}

fn clean_network(network: &Network, full_pipe: &Vec<Pos>) -> Vec<Vec<Option<PipeType>>> {
    network.iter().map(|row| 
        row.iter().map(|col| 
            if col.is_some() && full_pipe.contains(&col.unwrap().1) {
                Some(col.unwrap().0)
            } else {
                None
            }
        ).collect()
    ).collect()
}

fn find_inside_nodes(network: &Vec<Vec<Option<PipeType>>>) -> usize {
    let mut inside_spaces = 0;
    for (i, row) in network.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col.is_none() {
                let mut crossing_count = 0;
                let mut previous_turn = None;
                for c in 0..j {
                    if network[i][c] == Some(PipeType::NorthSouth) {
                        crossing_count += 1;
                    }

                    // coming from top and going to down with something in between counts as vertical pipe
                    // L[-]7
                    // F[-]J
                    if  network[i][c] == Some(PipeType::SouthEast) || network[i][c] == Some(PipeType::NorthWest)
                        || network[i][c] == Some(PipeType::SouthWest) || network[i][c] == Some(PipeType::NorthEast) {
                            if (previous_turn == Some(PipeType::NorthEast) && network[i][c] == Some(PipeType::SouthWest))
                                || (previous_turn == Some(PipeType::SouthEast) && network[i][c] == Some(PipeType::NorthWest)) {
                                    crossing_count += 1;
                                }
                        previous_turn = network[i][c];
                    }
                }
                
                if crossing_count % 2 == 0 {
                    // we are definitely outside, so we can go to the next position
                    continue;
                }

                // we found an inside space!
                inside_spaces += 1;
            }
        }
    }

    inside_spaces
}

fn interpolate_starting_pipe(dir: &str, start: &Pos, previous: &Pos) -> PipeType {
    let y = (previous.0 as i32) - (start.0 as i32);
    let x = (previous.1 as i32) - (start.1 as i32);
    let second_dir = match (y, x) {
        (0, 1) => "R",
        (0, -1) => "L",
        (1, 0) => "D",
        (-1, 0) => "U",
        _ => panic!("shouldn't happen"),
    };

    match (dir, second_dir) {
        ("U", "D") | ("D", "U") => PipeType::NorthSouth,
        ("L", "R") | ("R", "L") => PipeType::EastWest,
        ("L", "D") | ("D", "L") => PipeType::SouthWest,
        ("L", "U") | ("U", "L") => PipeType::NorthWest,
        ("R", "U") | ("U", "R") => PipeType::NorthEast,
        ("R", "D") | ("D", "R") => PipeType::SouthEast,
        y => panic!("couldn't find match for ({},{})", y.0, y.1),
    }
}

fn find_start(network: &Network) -> Pos {
    for row in network {
        for col in row {
            if let Some(pipe) = col {
                if pipe.0 == PipeType::Wildcard {
                    return pipe.1;
                }
            }
        }
    }
    panic!("Couldn't find start");
}

#[derive(Debug)]
#[derive(Copy, Clone, Hash)]
#[derive(PartialEq, Eq)]
pub struct Pipe(PipeType, Pos);

#[repr(u8)]
#[derive(Debug)]
#[derive(Copy, Clone, Hash)]
#[derive(PartialEq, Eq)]
pub enum PipeType {
    NorthSouth = b'|',
    EastWest = b'-',
    NorthEast = b'L',
    NorthWest = b'J',
    SouthWest = b'7',
    SouthEast = b'F',
    Wildcard = b'S',
}

impl Pos {
    fn next(&self, pipe: &PipeType, previous: &Pos) -> Option<Pos> {
        let neighbors = self.neighbors(pipe);

        if !neighbors.contains(previous) {
            // the previous position is not a neighbor, so we can't go anywhere obviously
            return None;
        }

        neighbors.into_iter().find(|x| x!= previous)
    }
    
    fn neighbors(&self, pipe: &PipeType) -> Vec<Pos> {
        let pos = self;
        let mut neighbors = vec![];
        if *pipe == PipeType::NorthSouth || *pipe == PipeType::NorthEast || *pipe == PipeType::NorthWest || *pipe == PipeType::Wildcard {
            if pos.0 != 0 {
                neighbors.push(Pos(pos.0 - 1, pos.1));
            }
        }
        if *pipe == PipeType::NorthWest || *pipe == PipeType::SouthWest || *pipe == PipeType::EastWest || *pipe == PipeType::Wildcard {
            if pos.1 != 0 {
                neighbors.push(Pos(pos.0, pos.1 - 1));
            }
        }
        if *pipe == PipeType::NorthEast || *pipe == PipeType::SouthEast || *pipe == PipeType::EastWest || *pipe == PipeType::Wildcard {
            neighbors.push(Pos(pos.0, pos.1 + 1));
        }
        if *pipe == PipeType::NorthSouth || *pipe == PipeType::SouthEast || *pipe == PipeType::SouthWest || *pipe == PipeType::Wildcard {
            neighbors.push(Pos(pos.0 + 1, pos.1));
        }
        neighbors
    }
}

impl FromStr for PipeType {
    type Err = ();

    fn from_str(s: &str) -> Result<PipeType, ()> {
        match s {
            "|" => Ok(PipeType::NorthSouth),
            "-" => Ok(PipeType::EastWest),
            "L" => Ok(PipeType::NorthEast),
            "J" => Ok(PipeType::NorthWest),
            "7" => Ok(PipeType::SouthWest),
            "F" => Ok(PipeType::SouthEast),
            "S" => Ok(PipeType::Wildcard),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for PipeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "{}", *self as u8 as char)
    }
}

#[derive(Debug)]
#[derive(Copy, Clone, Hash)]
#[derive(PartialEq, Eq)]
pub struct Pos(usize, usize);

pub type Network = Vec<Vec<Option<Pipe>>>;

#[allow(dead_code)]
fn print_pipes(pipes: &Network) {
    for row in pipes {
        for col in row {
            if let Some(pipe) = col  {
                print!("{}", pipe.0);
            } else {
                print!(" ");
            }
        }
        print!("\n")
    }
    io::stdout().flush().unwrap();
}

#[allow(dead_code)]
fn print_clean_pipes(pipes: &Vec<Vec<Option<PipeType>>>) {
    for row in pipes {
        for col in row {
            if let Some(pipe) = col  {
                print!("{}", pipe);
            } else {
                print!(" ");
            }
        }
        print!("\n")
    }
    io::stdout().flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_full_pipe_length_correctly() {
        let input: Vec<Vec<Option<PipeType>>> = vec![
            ["-", "L", "|", "F", "7"].iter().map(|x| PipeType::from_str(x).ok()).collect(),
            ["7", "S", "-", "7", "|"].iter().map(|x| PipeType::from_str(x).ok()).collect(),
            ["L", "|", "7", "|", "|"].iter().map(|x| PipeType::from_str(x).ok()).collect(),
            ["-", "L", "-", "J", "|"].iter().map(|x| PipeType::from_str(x).ok()).collect(),
            ["L", "|", "-", "J", "F"].iter().map(|x| PipeType::from_str(x).ok()).collect(),        
        ];

        let mut network: Network = input.iter().enumerate().map(|(i, row)|
            row.iter().enumerate().map(|(j, x)| if let Some(t) = x { Some(Pipe(*t, Pos(i, j))) } else { None }).collect()
        ).collect();

        let (full_pipe, pipe_network) = find_full_pipe(&mut network);

        print_pipes(&network);

        assert_eq!(full_pipe, 4);
    }

    #[test]
    fn finds_inside_spaces_correctly() {
        let mut network = read::read_all_lines("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........");

        let (full_pipe, pipe_network) = find_full_pipe(&mut network);

        print_pipes(&network);

        let network = clean_network(&network, &pipe_network);
        print_clean_pipes(&network);
        let inside_node_count = find_inside_nodes(&network);

        assert_eq!(inside_node_count, 4);
    }
}

mod read {
    use super::{Pipe, PipeType, Pos, Network};

    pub fn read_all_lines(i: &'static str) -> Network {
        let input: Vec<Vec<Option<PipeType>>> = i.lines()
        .map(|l| l.chars().map(|x| x.to_string().parse::<PipeType>().ok()).collect())
        .collect();

        let network: Network = input.iter().enumerate().map(|(i, row)|
            row.iter().enumerate().map(|(j, x)| if let Some(t) = x { Some(Pipe(*t, Pos(i, j))) } else { None }).collect()
        ).collect();

        network
    }
}
