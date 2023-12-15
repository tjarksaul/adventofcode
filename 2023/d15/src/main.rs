use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<String>) -> usize {
    input.iter().fold(0, |acc, cur| acc + hash(cur.to_string()))
}

fn part_2() -> usize {
    0
}

fn hash(string: String) -> usize {
    string.chars().fold(0, |acc, cur| (acc + cur as u8 as usize) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let input = "HASH".to_string();
        let hash = hash(input);
        assert_eq!(hash, 52);
    }

    #[test]
    fn test_part_1() {
        let input = vec!["rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7"].iter().map(|s| s.to_string()).collect();

        let result = part_1(&input);

        assert_eq!(result, 1320);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<String> {
        i.trim().split(",")
            .map(|l| l.to_string())
            .collect()
    }
}
