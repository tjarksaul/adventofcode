use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let part1 = get_num_total_possible_arrangements(&input);

    let part_2_input = create_part_2_input(&input);

    let part2 = get_num_total_possible_arrangements(&part_2_input);

    dbg!(part1, part2);
}

fn create_part_2_input(input: &Vec<(String, Vec<usize>)>) -> Vec<(String, Vec<usize>)> {
    input.iter().map(|(str, list)|
    (
        (0..5).fold(String::new(), |b, _| if b == "" { str.to_string() } else { b + "?" + str }),
        std::iter::repeat(
            list
        )
        .take(5)
        .flatten()
        .map(|c| *c)
        .collect()
    )
    ).collect()
}

fn get_num_total_possible_arrangements(input: &Vec<(String, Vec<usize>)>) -> usize {
    input.iter().fold(0, 
        |acc, (str, list)| { 
            hashmap().lock().unwrap().drain();
            acc + get_num_possible_arrangements(str.to_string(), list, 0, 0, 0)
        }
    )
}

fn hashmap() -> &'static Mutex<HashMap<(usize, usize, usize), usize>> {
    static ARRANGEMENT_MAP: OnceLock<Mutex<HashMap<(usize, usize, usize), usize>>> = OnceLock::new();
    ARRANGEMENT_MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

fn get_num_possible_arrangements(str: String, list: &Vec<usize>, i: usize, current_group: usize, in_group: usize) -> usize {
    if let Some(val) = hashmap().lock().unwrap().get(&(i, current_group, in_group)) {
        return *val;
    }

    if i == str.len() {
        if current_group == list.len() && in_group == 0 {
            return 1;
        } else if current_group == list.len() - 1 && in_group == list[current_group] {
            return 1;
        } else {
            return 0;
        }
    }
    let mut arrangements = 0;
    let str_vec: Vec<_> = str.chars().collect();
    for chr in ['.', '#'] {
        if str_vec[i] == chr || str_vec[i] == '?' {
            if chr == '.' && in_group == 0 {
                arrangements += get_num_possible_arrangements(str.to_string(), list, i + 1, current_group, 0);
            } else if chr == '.' && in_group > 0 && current_group < list.len() && list[current_group] == in_group {
                arrangements += get_num_possible_arrangements(str.to_string(), list, i + 1, current_group + 1, 0);
            } else if chr == '#' {
                arrangements += get_num_possible_arrangements(str.to_string(), list, i + 1, current_group, in_group + 1);
            }
        }
    }

    hashmap().lock().unwrap().insert((i, current_group, in_group), arrangements);

    arrangements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tests() {
        finds_arrangements_correctly();
        finds_all_arrangements_correctly();
        finds_all_arrangements_correctly_part_2();
    }

    fn finds_arrangements_correctly() {
        let input = ".#...#....###.".to_string();
        let list = vec![1, 1, 3];

        let possible_arrangements = get_num_possible_arrangements(input, &list, 0, 0, 0);

        assert_eq!(possible_arrangements, 1);

        let input = "???.###".to_string();
        let list = vec![1, 1, 3];

        let possible_arrangements = get_num_possible_arrangements(input, &list, 0, 0, 0);

        assert_eq!(possible_arrangements, 1);
    }

    fn get_input() -> Vec<(String, Vec<usize>)> {
        vec![
            ("???.###".to_string(), vec![1,1,3]),
            (".??..??...?##.".to_string(), vec![1,1,3]),
            ("?#?#?#?#?#?#?#?".to_string(), vec![1,3,1,6]),
            ("????.#...#...".to_string(), vec![4,1,1]),
            ("????.######..#####.".to_string(), vec![1,6,5]),
            ("?###????????".to_string(), vec![3,2,1]),
        ]
    }

    fn finds_all_arrangements_correctly() {
        let input = get_input();

        let num = get_num_total_possible_arrangements(&input);

        assert_eq!(num, 21);
    }

    fn finds_all_arrangements_correctly_part_2() {
        let i = get_input();
        let input = create_part_2_input(&i);

        dbg!(&input[0]);
        let num = get_num_total_possible_arrangements(&input);

        assert_eq!(num, 525152);
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
