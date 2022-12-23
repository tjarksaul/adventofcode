use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let mut elves = read::parse_all_lines(include_str!("../input.txt"));

    let free_space = calculate_free_space(&mut elves, 10);

    dbg!(free_space);

    let mut elves_2 = read::parse_all_lines(include_str!("../input.txt"));

    let move_count = find_first_move_without_change(&mut elves_2);

    dbg!(move_count);
}

#[derive(PartialEq, Debug, Hash, Eq, Clone)]
pub struct Elf(i64, i64);

impl Elf {
    fn adjacents(&self) -> HashSet<Elf> {
        let mut hash_set = HashSet::new();
        for x in [-1, 0, 1] {
            for y in [-1, 0, 1] {
                hash_set.insert(Elf(self.0 + y, self.1 + x));
            }
        }
        hash_set.remove(self);
        hash_set
    }
}

#[derive(PartialEq, Debug)]
enum Move {
    North,
    South,
    West,
    East,
}

impl Move {
    fn adjacents(&self, elf: &Elf) -> HashSet<Elf> {
        match self {
            Self::North => HashSet::from([
                Elf(elf.0 - 1, elf.1),
                Elf(elf.0 - 1, elf.1 - 1),
                Elf(elf.0 - 1, elf.1 + 1),
            ]),
            Self::South => HashSet::from([
                Elf(elf.0 + 1, elf.1),
                Elf(elf.0 + 1, elf.1 - 1),
                Elf(elf.0 + 1, elf.1 + 1),
            ]),
            Self::West => HashSet::from([
                Elf(elf.0, elf.1 - 1),
                Elf(elf.0 - 1, elf.1 - 1),
                Elf(elf.0 + 1, elf.1 - 1),
            ]),
            Self::East => HashSet::from([
                Elf(elf.0, elf.1 + 1),
                Elf(elf.0 - 1, elf.1 + 1),
                Elf(elf.0 + 1, elf.1 + 1),
            ]),
        }
    }

    fn new_position(&self, elf: &Elf) -> Elf {
        match self {
            Self::North => Elf(elf.0 - 1, elf.1),
            Self::South => Elf(elf.0 + 1, elf.1),
            Self::West => Elf(elf.0, elf.1 - 1),
            Self::East => Elf(elf.0, elf.1 + 1),
        }
    }
}

fn calculate_free_space(elves: &mut HashSet<Elf>, move_count: u64) -> u64 {
    let mut moves = VecDeque::from([Move::North, Move::South, Move::West, Move::East]);

    for _ in 0..move_count {
        let elves_vec: Vec<_> = elves.iter().collect();
        let mut new_positions = vec![];
        for i in 0..elves_vec.len() {
            let elf = elves_vec[i];

            let adjacents = elf.adjacents();
            let intersection: HashSet<_> = adjacents.intersection(&elves).collect();
            if intersection.len() == 0 {
                new_positions.push(elf.clone());
                // this elf does not have neighbors so we can just go on to the next elf
                continue;
            }

            let mut moved = false;
            for d in 0..moves.len() {
                let proposed_direction = &moves[d];
                let adjacents = proposed_direction.adjacents(elf);
                let intersection: HashSet<_> = adjacents.intersection(&elves).collect();
                if intersection.len() == 0 {
                    // we found a new position to move to
                    let new_position = proposed_direction.new_position(elf);
                    new_positions.push(new_position);
                    moved = true;
                    break;
                }
            }

            if !moved {
                new_positions.push(elf.clone());
            }
        }

        let mut new_elves_vec: Vec<_> = new_positions.iter().cloned().collect();
        for i in 0..new_positions.len() - 1 {
            // new_elves_vec.push(new_positions[i].clone());
            for j in i + 1..new_positions.len() {
                if new_positions[i] == new_positions[j] {
                    // we have two elves that want to move to the same location
                    // so we'll undo their moves
                    new_elves_vec[i] = elves_vec[i].clone();
                    new_elves_vec[j] = elves_vec[j].clone();
                }
            }
        }

        *elves = HashSet::from_iter(new_elves_vec.iter().cloned());

        let first = moves.pop_front().unwrap();
        moves.push_back(first);
    }

    // and here we need to figure out the size of the square
    let min_x = elves.iter().map(|e| e.1).min().unwrap();
    let max_x = elves.iter().map(|e| e.1).max().unwrap();
    let min_y = elves.iter().map(|e| e.0).min().unwrap();
    let max_y = elves.iter().map(|e| e.0).max().unwrap();

    let size = (max_y + 1 - min_y) * (max_x + 1 - min_x);
    let size = size as u64;

    size - (elves.len() as u64)
}

fn find_first_move_without_change(elves: &mut HashSet<Elf>) -> u64 {
    let mut moves = VecDeque::from([Move::North, Move::South, Move::West, Move::East]);

    let mut move_count = 0;
    loop {
        move_count += 1;
        let elves_vec: Vec<_> = elves.iter().collect();
        let mut new_positions = vec![];
        for i in 0..elves_vec.len() {
            let elf = elves_vec[i];

            let adjacents = elf.adjacents();
            let intersection: HashSet<_> = adjacents.intersection(&elves).collect();
            if intersection.len() == 0 {
                new_positions.push(elf.clone());
                // this elf does not have neighbors so we can just go on to the next elf
                continue;
            }

            let mut moved = false;
            for d in 0..moves.len() {
                let proposed_direction = &moves[d];
                let adjacents = proposed_direction.adjacents(elf);
                let intersection: HashSet<_> = adjacents.intersection(&elves).collect();
                if intersection.len() == 0 {
                    // we found a new position to move to
                    let new_position = proposed_direction.new_position(elf);
                    new_positions.push(new_position);
                    moved = true;
                    break;
                }
            }

            if !moved {
                new_positions.push(elf.clone());
            }
        }

        let mut new_elves_vec: Vec<_> = new_positions.iter().cloned().collect();
        for i in 0..new_positions.len() - 1 {
            // new_elves_vec.push(new_positions[i].clone());
            for j in i + 1..new_positions.len() {
                if new_positions[i] == new_positions[j] {
                    // we have two elves that want to move to the same location
                    // so we'll undo their moves
                    new_elves_vec[i] = elves_vec[i].clone();
                    new_elves_vec[j] = elves_vec[j].clone();
                }
            }
        }

        let new_elves = HashSet::from_iter(new_elves_vec.iter().cloned());
        if *elves == new_elves {
            return move_count;
        }
        *elves = new_elves;

        let first = moves.pop_front().unwrap();
        moves.push_back(first);
    }
}

#[allow(dead_code)]
fn print_elves(elves: &HashSet<Elf>) {
    let min_x = elves.iter().map(|e| e.1).min().unwrap();
    let max_x = elves.iter().map(|e| e.1).max().unwrap();
    let min_y = elves.iter().map(|e| e.0).min().unwrap();
    let max_y = elves.iter().map(|e| e.0).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let elf = Elf(y, x);
            if elves.contains(&elf) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> HashSet<Elf> {
        HashSet::from([
            Elf(0, 4),
            Elf(1, 2),
            Elf(1, 3),
            Elf(1, 4),
            Elf(1, 6),
            Elf(2, 0),
            Elf(2, 4),
            Elf(2, 6),
            Elf(3, 1),
            Elf(3, 5),
            Elf(3, 6),
            Elf(4, 0),
            Elf(4, 2),
            Elf(4, 3),
            Elf(4, 4),
            Elf(5, 0),
            Elf(5, 1),
            Elf(5, 3),
            Elf(5, 5),
            Elf(5, 6),
            Elf(6, 1),
            Elf(6, 4),
        ])
    }

    #[test]
    fn test_finds_correct_amount_free_space() {
        let mut elves = get_input();

        let free_space = calculate_free_space(&mut elves, 10);

        assert_eq!(free_space, 110);
    }

    #[test]
    fn test_finds_correct_amount_of_moves() {
        let mut elves = get_input();

        let move_count = find_first_move_without_change(&mut elves);

        assert_eq!(move_count, 20);
    }

    #[test]
    fn test_parser() {
        let text = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        let expected = get_input();

        let parsed = read::parse_all_lines(text);

        assert_eq!(parsed, expected);
    }
}

mod read {
    use super::Elf;
    use std::collections::HashSet;

    pub fn parse_all_lines(input: &str) -> HashSet<Elf> {
        let mut elves = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                if chr == '#' {
                    let y = y as i64;
                    let x = x as i64;
                    elves.insert(Elf(y, x));
                }
            }
        }

        elves
    }
}
