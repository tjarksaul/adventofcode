use std::fmt::Formatter;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let full_pipe = find_full_pipe(&input);

    dbg!(full_pipe);
}

fn find_full_pipe(input: &Network) -> usize {
    let start = find_start(&input);

    for pos in [
        Pos(start.0 + 1, start.1 + 0), 
        Pos(start.0 -1, start.1 + 0), 
        Pos(start.0 + 0, start.1 -1), 
        Pos(start.0 + 0, start.1 + 1)
    ] {
        let mut previous = start;
        let mut position: Option<Pos> = Some(pos);
        let mut steps = 1;

        while position.is_some() {
            let pos = position.unwrap();
            if let Some(pipe) = input[pos.0][pos.1] {
                let pipe = pipe.0;
                let next = pos.next(&pipe, &previous);
                if let Some(cur) = next {
                    steps += 1;
                    if cur == start {
                        return steps / 2;
                    }
                }
                previous = pos;
                position = next;
            } else { break; }
        }
    }
    
    panic!("Nothing found");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_sum_correctly() {
        let input: Vec<Vec<Option<PipeType>>> = vec![
            ["-", "L", "|", "F", "7"].iter().map(|x| PipeType::from_str(x).ok()).collect(),
            ["7", "S", "-", "7", "|"].iter().map(|x| PipeType::from_str(x).ok()).collect(),
            ["L", "|", "7", "|", "|"].iter().map(|x| PipeType::from_str(x).ok()).collect(),
            ["-", "L", "-", "J", "|"].iter().map(|x| PipeType::from_str(x).ok()).collect(),
            ["L", "|", "-", "J", "F"].iter().map(|x| PipeType::from_str(x).ok()).collect(),        
        ];

        let network: Network = input.iter().enumerate().map(|(i, row)|
            row.iter().enumerate().map(|(j, x)| if let Some(t) = x { Some(Pipe(*t, Pos(i, j))) } else { None }).collect()
        ).collect();

        // print_pipes(&network);

        let full_pipe = find_full_pipe(&network);

        assert_eq!(full_pipe, 4);
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
