use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2(&input);

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<String>) -> usize {
    input.iter().fold(0, |acc, cur| acc + hash(cur.to_string()))
}

fn part_2(input: &Vec<String>) -> usize {
    let instructions = input.iter().map(|s| to_instruction(s.to_string()));

    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    for instruction in instructions {
        match instruction.1 {
            Action::Add(label, focus) => {
                let index = boxes[instruction.0].iter().position(|r| r.0 == label);
                if let Some(idx) = index {
                    boxes[instruction.0][idx] = (label, focus);
                } else {
                    boxes[instruction.0].push((label, focus));
                }
            }
            Action::Remove(label) => {
                let index = boxes[instruction.0].iter().position(|r| r.0 == label);
                if let Some(idx) = index {
                    boxes[instruction.0].remove(idx);
                }
            }
        }
    }

    boxes.iter().enumerate().fold(0, |acc, 
        (bx_i, bx)| acc + bx.iter().enumerate().fold(0, | bx_acc, (i, cur)| 
            bx_acc + (bx_i + 1) * (i + 1) * cur.1
        )
    )
}

fn hash(string: String) -> usize {
    string.chars().fold(0, |acc, cur| (acc + cur as u8 as usize) * 17 % 256)
}

fn to_instruction(ins: String) -> Instruction {
    if ins.contains("=") {
        let split: Vec<_> = ins.split("=").collect();
        assert!(split.len() == 2);
        let label = split[0].to_string();
        let hash = hash(label.to_string());
        let amount: usize = split[1].parse().unwrap();
        Instruction(hash, Action::Add(label, amount))
    } else {
        let split: Vec<_> = ins.split("-").collect();
        assert!(split.len() == 2);
        let label = split[0].to_string();
        let hash = hash(label.to_string());
        Instruction(hash, Action::Remove(label))
    }
}

struct Instruction(usize, Action);
enum Action {
    Remove(String),
    Add(String, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        vec!["rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7"].iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_hash() {
        let input = "HASH".to_string();
        let hash = hash(input);
        assert_eq!(hash, 52);
    }

    #[test]
    fn test_part_1() {
        let input = get_input();

        let result = part_1(&input);

        assert_eq!(result, 1320);
    }

    #[test]
    fn test_part_2() {
        let input = get_input();

        let result = part_2(&input);

        assert_eq!(result, 145);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<String> {
        i.trim().split(",")
            .map(|l| l.to_string())
            .collect()
    }
}
