pub fn main() {
    let input = read::read_input("input.txt".to_string());
    let priorities = calculate_priorities(input);

    println!("Sum of priorities (1): {}", priorities);
}

fn calculate_priorities(rucksacks: Vec<Vec<char>>) -> i64 {
    return rucksacks
        .iter()
        .map(|rucksack| calculate_priority(rucksack))
        .fold(0, |a, b| a + b);
}

fn calculate_priority(rucksack: &Vec<char>) -> i64 {
    let compartment_1 = &rucksack[..=rucksack.len() / 2];
    let compartment_2 = &rucksack[rucksack.len() / 2..];

    let duplicate = compartment_1
        .iter()
        .find(|item| compartment_2.iter().any(|item2| *item == item2))
        .unwrap();

    let ascii = *duplicate as u32;
    let priority = if ascii >= 97 { ascii - 96 } else { ascii - 38 };

    return priority.into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_correctly() {
        let rucksacks = vec![
            vec![
                'v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r', 'h', 'c', 's', 'F',
                'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p',
            ],
            vec![
                'j', 'q', 'H', 'R', 'N', 'q', 'R', 'j', 'q', 'z', 'j', 'G', 'D', 'L', 'G', 'L',
                'r', 's', 'F', 'M', 'f', 'F', 'Z', 'S', 'r', 'L', 'r', 'F', 'Z', 's', 'S', 'L',
            ],
            vec![
                'P', 'm', 'm', 'd', 'z', 'q', 'P', 'r', 'V', 'v', 'P', 'w', 'w', 'T', 'W', 'B',
                'w', 'g',
            ],
            vec![
                'w', 'M', 'q', 'v', 'L', 'M', 'Z', 'H', 'h', 'H', 'M', 'v', 'w', 'L', 'H', 'j',
                'b', 'v', 'c', 'j', 'n', 'n', 'S', 'B', 'n', 'v', 'T', 'Q', 'F', 'n',
            ],
            vec![
                't', 't', 'g', 'J', 't', 'R', 'G', 'J', 'Q', 'c', 't', 'T', 'Z', 't', 'Z', 'T',
            ],
            vec![
                'C', 'r', 'Z', 's', 'J', 's', 'P', 'P', 'Z', 's', 'G', 'z', 'w', 'w', 's', 'L',
                'w', 'L', 'm', 'p', 'w', 'M', 'D', 'w',
            ],
        ];

        let priorities = calculate_priorities(rucksacks);

        assert_eq!(priorities, 157);
    }
}

mod read {
    use std::fs;

    pub fn read_input(fname: String) -> Vec<Vec<char>> {
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

        let splits: Vec<&str> = contents.split("\n").collect();

        return splits.iter().map(|split| split.chars().collect()).collect();
    }
}
