fn main() {
    let (mut crates, moves) = read::read_input("input.txt".to_string());
    let crate_tops = move_crates(&mut crates, moves);

    println!("Got following crates: {}", crate_tops);
}

fn move_crates(crates: &mut Vec<Vec<char>>, moves: Vec<Move>) -> String {
    for mov in moves {
        for _ in 0..mov.amount {
            let crat = crates[mov.source - 1].remove(0);
            crates[mov.destination - 1].insert(0, crat);
        }
    }

    let mut string: String = String::from("");

    for stack in crates {
        let Some(chr) = stack.first() else { continue };

        string.push(*chr);
    }

    return string;
}

pub struct Move {
    amount: i32,
    source: usize,
    destination: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moves_crates_correctly() {
        let mut stacks = vec![
            vec!['N', 'Z'],
            vec!['D', 'C', 'M'],
            vec!['P'],
        ];
        let moves = vec![
            Move { amount: 1, source: 2, destination: 1 },
            Move { amount: 3, source: 1, destination: 3 },
            Move { amount: 2, source: 2, destination: 1 },
            Move { amount: 1, source: 1, destination: 2 },
        ];

        let crate_string = move_crates(&mut stacks, moves);

        assert_eq!(crate_string, "CMZ");
    }
}

mod read {
    use super::Move;
    use std::fs;

    pub fn read_input(fname: String) -> (Vec<Vec<char>>, Vec<Move>) {
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

        let splits: Vec<Vec<&str>> = contents.split("\n\n").map(|part| part.lines().collect()).collect();

        return (parse_crates(&splits[0]), parse_moves(&splits[1]));
    }

    fn parse_crates(lines: &Vec<&str>) -> Vec<Vec<char>> {
        let mut crates: Vec<Vec<char>> = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        for line in lines {
            for crt in 0..9 {
                let chr = line.chars().nth((crt * 4) + 1).unwrap();
                if chr != ' ' {
                    crates[crt].push(chr);
                }
            }
        }
        return crates;
    }

    fn parse_moves(lines: &Vec<&str>) -> Vec<Move> {
        return lines.into_iter().map(parse_move).collect()
    }
    
    fn parse_move(line: &&str) -> Move {
        let splits: Vec<&str> = line.split(' ').collect();
        let amount: i32 = splits[1].parse::<>().unwrap();
        let source: usize = splits[3].parse::<>().unwrap();
        let destination: usize = splits[5].parse::<>().unwrap();

        return Move { amount, source, destination };
    }
}
