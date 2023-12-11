use std::cmp;
use std::fmt::Formatter;
use std::io;
use std::io::Write;

fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let shortest_paths = find_shortest_paths(&input, 1000000);

    dbg!(shortest_paths);
}

fn find_shortest_paths(universe: &Vec<Vec<bool>>, expansion_factor: usize) -> usize {
    let mut path_lengths = 0;

    let galaxies = get_galaxies(&universe);
    for i in 0..(galaxies.len() - 1) {
        for j in (i+1)..galaxies.len() {
            let mut path_length = galaxies[i].distance(&galaxies[j]);
            let empty_spaces = find_empty_spaces(&galaxies[i], &galaxies[j], &universe);
            path_length += empty_spaces * (expansion_factor - 1);
            path_lengths += path_length;
        }
    }
    path_lengths
}

fn find_empty_spaces(p1: &Pos, p2: &Pos, universe: &Vec<Vec<bool>>) -> usize {
    let mut empty_count = 0;
    let min = cmp::min(p1.0, p2.0);
    let max = cmp::max(p1.0, p2.0);
    for i in (min + 1)..max {
        if is_empty_row(&universe[i]) {
            empty_count += 1;
        }
    }

    let min = cmp::min(p1.1, p2.1);
    let max = cmp::max(p1.1, p2.1);
    for i in (min + 1)..max {
        if is_empty_column(&universe, i) {
            empty_count += 1;
        }
    }

    empty_count
}

fn is_empty_row(row: &Vec<bool>) -> bool {
    row.iter().all(|x| !x)
}

fn is_empty_column(universe: &Vec<Vec<bool>>, col: usize) -> bool {
    for i in 0..universe.len() {
        if universe[i][col] {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn print_universe(universe: &Vec<Vec<bool>>) {
    for row in universe {
        for column in row {
            print!("{}", if *column { "#" } else { "." });
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(
    // y
    usize,
    // x
    usize
);

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
        let universe: Vec<Vec<bool>> = vec![
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

        let sum = find_shortest_paths(&universe, 2);

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