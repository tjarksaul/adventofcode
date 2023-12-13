use std::cmp;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Vec<Vec<u8>>>) -> usize {
    let (horizontal, vertical) = input.iter().fold((0, 0), |acc, cur| {
        let reflection = find_perfect_reflection(&cur);
        match reflection {
            Reflection::Horizontal(val) => (acc.0 + val, acc.1),
            Reflection::Vertical(val) => (acc.0, acc.1 + val),
        }
    });

    100 * horizontal + vertical
}

fn find_perfect_reflection(input: &Vec<Vec<u8>>) -> Reflection {
    let (best_horizontal, missing_horizontal) = find_reflection_on_axis(input);

    let transposed = (0..input[0].len())
        .map(|i| input.iter().map(|inner| inner[i].clone()).collect::<Vec<u8>>())
        .collect();
    let (best_vertical, missing_vertical) = find_reflection_on_axis(&transposed);

    if missing_horizontal.unwrap_or(9999999) > missing_vertical.unwrap_or(9999999) {
        return Reflection::Vertical(best_vertical.unwrap());
    } else {
        return Reflection::Horizontal(best_horizontal.unwrap());
    }
}

fn find_reflection_on_axis(input: &Vec<Vec<u8>>) -> (Option<usize>, Option<usize>) {
    let mut best = None;
    let mut missing = None;
    for i in 1..input.len() {
        let before: Vec<_> = input[0..i].into_iter().rev().collect();
        let after = &input[i..];
        
        let mut completed = false;
        let mut j = 0;
        while before[j] == &after[j] {
            j += 1;
            if j == cmp::min(before.len(), after.len()) {
                completed = true;
                break;
            }
        }
        if completed {
            let miss = cmp::max(before.len(), after.len()) - cmp::min(before.len(), after.len());
            if missing.unwrap_or(9999999) > miss {
                best = Some(i);
                missing = Some(miss);
            }
        }
    }

    (best, missing)
}

fn part_2() -> usize {
    0
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Vec<Vec<u8>>> {
        vec![
            vec![
                vec![b'#', b'.', b'#', b'#', b'.', b'.', b'#', b'#', b'.'],
                vec![b'.', b'.', b'#', b'.', b'#', b'#', b'.', b'#', b'.'],
                vec![b'#', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'#'],
                vec![b'#', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'#'],
                vec![b'.', b'.', b'#', b'.', b'#', b'#', b'.', b'#', b'.'],
                vec![b'.', b'.', b'#', b'#', b'.', b'.', b'#', b'#', b'.'],
                vec![b'#', b'.', b'#', b'.', b'#', b'#', b'.', b'#', b'.'],
            ],
            vec![
                vec![b'#', b'.', b'.', b'.', b'#', b'#', b'.', b'.', b'#'],
                vec![b'#', b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'#'],
                vec![b'.', b'.', b'#', b'#', b'.', b'.', b'#', b'#', b'#'],
                vec![b'#', b'#', b'#', b'#', b'#', b'.', b'#', b'#', b'.'],
                vec![b'#', b'#', b'#', b'#', b'#', b'.', b'#', b'#', b'.'],
                vec![b'.', b'.', b'#', b'#', b'.', b'.', b'#', b'#', b'#'],
                vec![b'#', b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'#'],
            ],
        ]
    }

    #[test]
    fn finds_perfect_reflection() {
        let inputs = get_input();

        let reflection = find_perfect_reflection(&inputs[0]);

        assert_eq!(reflection, Reflection::Vertical(5));

        let reflection = find_perfect_reflection(&inputs[1]);

        assert_eq!(reflection, Reflection::Horizontal(4));
    }

    #[test]
    fn finds_perfect_reflections() {
        let inputs = get_input();

        let total = part_1(&inputs);

        assert_eq!(total, 405);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<Vec<Vec<u8>>> {
        i.split("\n\n").map(|block|
            block.lines()
                .map(|l| l.chars().into_iter().map(|c| c as u8).collect())
                .collect()
        ).collect()
    }
}
