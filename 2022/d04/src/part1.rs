pub fn main() {
    let input = read::read_input("input.txt".to_string());
    let fully_overlapping_pairs = count_fully_overlapping_pairs(input);

    println!("Amount of fully overlapping pairs: {}", fully_overlapping_pairs);
}

pub type Pair = ((i32, i32), (i32, i32));

fn count_fully_overlapping_pairs(pairs: Vec<Pair>) -> i32 {
    return pairs.iter().map(is_overlapping_pair).fold(0, |a, b| a + (b as i32));
}

fn is_overlapping_pair(pair: &Pair) -> bool {
    return (pair.0.0 >= pair.1.0 && pair.0.1 <= pair.1.1) || (pair.0.0 <= pair.1.0 && pair.0.1 >= pair.1.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_correctly() {
        let pairs = vec![
            ((2, 4),(6, 8)),
            ((2, 3),(4, 5)),
            ((5, 7),(7, 9)),
            ((2, 8),(3, 7)),
            ((6, 6),(4, 6)),
            ((2, 6),(4, 8)),
        ];

        let fully_overlapping_pairs = count_fully_overlapping_pairs(pairs);

        assert_eq!(fully_overlapping_pairs, 2);
    }
}

mod read {
    use std::fs;
    use super::Pair;

    pub fn read_input(fname: String) -> Vec<Pair> {
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

        let splits: Vec<&str> = contents.split("\n").collect();

        return splits.iter().map(parse_pair).collect();
    }

    fn parse_pair(string: &&str) -> Pair {
        let split: Vec<(i32, i32)> = string.split(",").map(parse_range).collect();

        return (split[0], split[1]);
    }

    fn parse_range(string: &str) -> (i32, i32) {
        let split: Vec<i32> = string.split("-").map(|it| it.parse::<>().unwrap()).collect();

        return (split[0], split[1]);
    }
}
