use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let num = get_num_total_possible_arrangements(&input);

    dbg!(num);
}

fn get_num_total_possible_arrangements(input: &Vec<(String, Vec<usize>)>) -> usize {
    input.iter().fold(0, 
        |acc, (str, list)| acc + get_num_possible_arrangements(str.to_string(), list)
    )
}

fn get_num_possible_arrangements(str: String, list: &Vec<usize>) -> usize {
    let mut arrangements = 0;
    let mut groups = vec![];
    let mut in_group: Option<usize> = None;
    let mut options = vec![];
    for (i, chr) in str.chars().enumerate() {
        if chr == '.' {
            if in_group.is_some() {
                groups.push((in_group.unwrap(), i, i - in_group.unwrap()));
            }
            in_group = None;
        } else if chr == '#' {
            if in_group.is_none() {
                in_group = Some(i);
            }
        } else if chr == '?' {
            if in_group.is_some() {
                groups.push((in_group.unwrap(), i, i - in_group.unwrap()));
            }
            options.push(i);
        }
    }
    if in_group.is_some() {
        groups.push((in_group.unwrap(), str.len() - 1, str.len() - 1 - in_group.unwrap()));
    }

    if options.len() == 0 {
        if groups.iter().map(|(_, _, l)| *l).collect::<Vec<_>>() == *list {
            return 1;
        } else {
            panic!("we shouldn't go here");
        }
    }

    let combinations = get_possible_combinations(options.len());

    for v in combinations.iter() {
        let replacements /* :  HashMap<_, _> */ = options.iter().enumerate().map(|(i, pos)| (*pos, if v[i] { '#' } else { '.' })).collect();
        let possible_groups = find_groups(&str, &replacements);
        if possible_groups == *list {
            arrangements += 1;
        }
    }

    arrangements
}

fn find_groups(str: &String, replacements: &HashMap<usize, char>) -> Vec<usize> {
    let mut groups = vec![];
    let mut in_group: Option<usize> = None;
    let mut options = vec![];
    for (i, chr) in str.chars().enumerate() {
        if chr == '.' {
            if in_group.is_some() {
                groups.push((in_group.unwrap(), i - 1, i - in_group.unwrap()));
            }
            in_group = None;
        } else if chr == '#' {
            if in_group.is_none() {
                in_group = Some(i);
            }
        } else if chr == '?' {
            let replacement = replacements.get(&i).unwrap();
            if *replacement == '#' {
                if in_group.is_none() {
                    in_group = Some(i);
                }    
            } else {
                if in_group.is_some() {
                    groups.push((in_group.unwrap(), i - 1, i - in_group.unwrap()));
                }
                in_group = None;
            }
            options.push(i);
        }
    }
    if in_group.is_some() {
        groups.push((in_group.unwrap(), str.len() - 1, str.len() - in_group.unwrap()));
    }

    groups.iter().map(|(_, _, l)| *l).collect()
}

fn get_possible_combinations(options_length: usize) -> Vec<Vec<bool>> {
    let bools = vec![false, true];

    if options_length == 0 {
        return vec![];
    } else if options_length == 1 {
        return vec![bools];
    }

    (2..options_length).fold(
        bools.iter().cartesian_product(bools.iter()).map(|(&a, &b)| vec![a.to_owned(), b]).collect(),
        |acc, _| acc.into_iter().cartesian_product(bools.iter())
            .map(|(a, b)| { let mut c = a.to_owned(); c.push(*b); c }
        ).collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_arrangements_correctly() {
        let input = ".#...#....###.".to_string();
        let list = vec![1, 1, 3];

        let possible_arrangements = get_num_possible_arrangements(input, &list);

        assert_eq!(possible_arrangements, 1);

        let input = "???.###".to_string();
        let list = vec![1, 1, 3];

        let possible_arrangements = get_num_possible_arrangements(input, &list);

        assert_eq!(possible_arrangements, 1);
    }

    #[test]
    fn finds_all_arrangements_correctly() {
        let input = vec![
            ("???.###".to_string(), vec![1,1,3]),
            (".??..??...?##.".to_string(), vec![1,1,3]),
            ("?#?#?#?#?#?#?#?".to_string(), vec![1,3,1,6]),
            ("????.#...#...".to_string(), vec![4,1,1]),
            ("????.######..#####.".to_string(), vec![1,6,5]),
            ("?###????????".to_string(), vec![3,2,1]),
        ];

        let num = get_num_total_possible_arrangements(&input);

        assert_eq!(num, 21);
    }
}

mod read {
    pub fn read_all_lines(i: &'static str) -> Vec<(String, Vec<usize>)> {
        i.lines()
            .map(|l| {
                let parts = l.split(" ").collect::<Vec<_>>();
                if parts.len() != 2 {
                    panic!("Can't parse line {}", l);
                }
                (parts[0].to_string(), parts[1].split(",").map(|c| c.parse().unwrap()).collect())
            })
            .collect()
    }

}