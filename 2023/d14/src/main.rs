use std::collections::HashMap;
use std::error::Error;
use itertools::Itertools;
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let mut mutable = input.iter().map(|r| r.to_vec()).collect();
    let part1 = part_1(&mut mutable);

    let mut mutable = input.iter().map(|r| r.to_vec()).collect();
    let part2 = part_2(&mut mutable);

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &mut Vec<Vec<u8>>) -> usize {
    drop_stones(input, Direction::North);
    // print_result(&input);
    calculate_weight(&input)
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn drop_stones(input: &mut Vec<Vec<u8>>, direction: Direction) {
    if direction == Direction::North || direction == Direction::South {
        let rows = input.len();
        let outer_range: Box<dyn Iterator<Item=usize>> = if direction == Direction:: North { 
            Box::new(1..input.len()) 
        } else { 
            Box::new((0..(input.len() - 1)).rev())
        };
        for i in outer_range {
            for j in 0..input[i].len() {
                if input[i][j] == b'O' {
                    let mut target = i;
                    let target_range: Box<dyn Iterator<Item=usize>> = if direction == Direction::North {
                        Box::new((0..i).rev()) 
                    } else {
                        Box::new((i+1)..rows)
                    };
                    for k in target_range {
                        if input[k][j] == b'.' {
                            target = k;
                        } else if input[k][j] == b'#' || input[k][j] == b'O' {
                            break;
                        }
                    }
                    input[i][j] = b'.';
                    input[target][j] = b'O';
                }
            }
        }
    } else {
        let outer_range: Box<dyn Iterator<Item=usize>> = if direction == Direction::West {
            Box::new(1..input[0].len())
        } else {
            Box::new((0..(input[0].len() -1)).rev())
        };
        let cols = input[0].len();
        for j in outer_range {
            for i in 0..input.len() {
                if input[i][j] == b'O' {
                    let mut target = j;
                    let target_range: Box<dyn Iterator<Item=usize>> = if direction == Direction::West {
                        Box::new((0..j).rev()) 
                    } else {
                        Box::new((j+1)..cols)
                    };
                    for k in target_range {
                        if input[i][k] == b'.' {
                            target = k;
                        } else if input[i][k] == b'#' || input[i][k] == b'O' {
                            break;
                        }
                    }
                    input[i][j] = b'.';
                    input[i][target] = b'O';
                }
            }
        }
    }
}

fn calculate_weight(input: &Vec<Vec<u8>>) -> usize {
    input.iter().rev().enumerate().fold(0, |acc, (i, cur)| 
        acc + ((i + 1) * cur.iter().fold(0, |inner_acc, chr| inner_acc + if *chr == b'O' { 1 } else { 0 }))
    )
}

fn print_result(input: &Vec<Vec<u8>>) {
    for row in input {
        for chr in row {
            print!("{}", *chr as char);
        }
        print!("\n");
    }
}

fn part_2(input: &mut Vec<Vec<u8>>) -> usize {
    let mut entry_map: HashMap<String, usize> = HashMap::new();
    let target = 1000000000;
    let mut cycle = 0;
    while cycle < target {
        cycle += 1;
        println!("Cycle {}", cycle);
        drop_stones(input, Direction::North);
        drop_stones(input, Direction::West);
        drop_stones(input, Direction::South);
        drop_stones(input, Direction::East);

        let entry = input.iter().map(|r| str::from_utf8(r).unwrap()).join("").to_string();

        if let Some(last) = entry_map.get(&entry) {
            let cycle_length = cycle - last;
            let multiplier = (target - cycle) / cycle_length;
            cycle += multiplier * cycle_length;
        }

        entry_map.insert(entry, cycle);
    }
    calculate_weight(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        assert_eq!(0, 1);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<Vec<u8>> {
        i.lines()
            .map(|l| l.chars().map(|c| c as u8).collect())
            .collect()
    }
}
