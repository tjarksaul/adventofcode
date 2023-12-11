use std::fmt::Formatter;
use std::io;
use std::io::Write;

fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let expand_universe = expand_universe(&input);
    let shortest_paths = find_shortest_paths(&expand_universe);

    dbg!(shortest_paths);
}

fn find_shortest_paths(universe: &Vec<Vec<bool>>) -> usize {
    let mut path_lengths = 0;

    let galaxies = get_galaxies(&universe);
    for i in 0..(galaxies.len() - 1) {
        for j in (i+1)..galaxies.len() {
            let path_length = galaxies[i].distance(&galaxies[j]);
            // println!("{} -> {}: {}", galaxies[i], galaxies[j], path_length);
            path_lengths += path_length;
        }
    }
    path_lengths
}

fn print_universe(universe: &Vec<Vec<bool>>) {
    for row in universe {
        for column in row {
            print!("{}", if *column { "#" } else { "." });
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
}

fn expand_universe(universe: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result = vec![];
    for i in 0..universe.len() {
        result.push(universe[i].to_vec());
        if universe[i].iter().all(|x| !x) {
            // empty row, double it and give it to the next person
            result.push(universe[i].to_vec());
        }
    }
    let mut i = 0;
    while i < result[0].len() {
        let mut is_empty = true;
        for j in 0..result.len() {
            if result[j][i] {
                // no empty column, continue next loop
                is_empty = false;
                break;
            }
        }
        if is_empty {
            for j in 0..result.len() {
                let val = result[j][i];
                result[j].insert(i + 1, val);
            }
            // so that we increase i by two to skip the already added column
            i += 1;
        }
        i += 1;
    }

    result
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
  fn distance(&self, other: &Pos) -> usize {
    (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as usize
  }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn get_galaxies(universe: &Vec<Vec<bool>>) -> Vec<Pos> {
    let mut positions = vec![];
    for i in 0..universe.len() {
        for j in 0..universe[i].len() {
            if universe[i][j] {
                positions.push(Pos(i, j));
            }
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_sum_correctly() {
        let input: Vec<Vec<bool>> = vec![
            ['.', '.', '.', '#', '.', '.', '.', '.', '.', '.'].iter().map(|x| *x == '#').collect(),
            ['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'].iter().map(|x| *x == '#').collect(),
            ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'].iter().map(|x| *x == '#').collect(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'].iter().map(|x| *x == '#').collect(),
            ['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'].iter().map(|x| *x == '#').collect(),
            ['.', '#', '.', '.', '.', '.', '.', '.', '.', '.'].iter().map(|x| *x == '#').collect(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'].iter().map(|x| *x == '#').collect(),
            ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'].iter().map(|x| *x == '#').collect(),
            ['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'].iter().map(|x| *x == '#').collect(),
            ['#', '.', '.', '.', '#', '.', '.', '.', '.', '.'].iter().map(|x| *x == '#').collect(),
        ];

        let expand_universe = expand_universe(&input);
        let sum = find_shortest_paths(&expand_universe);

        assert_eq!(sum, 374);
    }
}

mod read {
    pub fn read_all_lines(i: &'static str) -> Vec<Vec<bool>> {
        i.lines()
        .map(|l| l.chars().map(|x| x == '#').collect())
        .collect()
    }
}