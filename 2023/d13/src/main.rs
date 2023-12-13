use std::cmp;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input)?;

    let part2 = part_2(&input)?;

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Vec<Vec<u8>>>) -> Result<usize, Box<dyn Error>> {
    run(input, false)
}

fn part_2(input: &Vec<Vec<Vec<u8>>>) -> Result<usize, Box<dyn Error>> {
    run(input, true)
}

fn run(input: &Vec<Vec<Vec<u8>>>, part_2: bool) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;
    for cur in input {
        let reflection = find_perfect_reflection(&cur, part_2)?;
        match reflection {
            Reflection::Horizontal(val) => sum += 100 * val,
            Reflection::Vertical(val) => sum += val,
        };
    }

    Ok(sum)
}

fn find_perfect_reflection(input: &Vec<Vec<u8>>, part_2: bool) -> Result<Reflection, Box<dyn Error>> {
    let horizontal = find_reflection_on_axis(input, part_2);
    if horizontal.is_some() {
        return Ok(Reflection::Horizontal(horizontal.unwrap()));
    }

    let transposed = (0..input[0].len())
        .map(|i| input.iter().map(|inner| inner[i].clone()).collect::<Vec<u8>>())
        .collect();
    let vertical = find_reflection_on_axis(&transposed, part_2);

    if vertical.is_some() {
        return Ok(Reflection::Vertical(vertical.unwrap()));
    }
    Err("Couldn't find a reflection")?
}

fn find_reflection_on_axis(input: &Vec<Vec<u8>>, part_2: bool) -> Option<usize> {
    let mut best = None;
    let mut missing = None;
    for i in 1..input.len() {
        let before: Vec<_> = input[0..i].into_iter().rev().collect();
        let after = &input[i..];
        
        let mut differences = 0;
        for j in 0..cmp::min(before.len(), after.len()) {
            for k in 0..before[j].len() {
                if before[j][k] != after[j][k] {
                    differences += 1;
                }
            }
        }
        if differences == if part_2 { 1 } else { 0 } {
            let miss = cmp::max(before.len(), after.len()) - cmp::min(before.len(), after.len());
            if missing.unwrap_or(9999999) > miss {
                best = Some(i);
                missing = Some(miss);
            }
        }
    }

    best
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

        let reflection = find_perfect_reflection(&inputs[0], false);

        assert_eq!(reflection, Reflection::Vertical(5));

        let reflection = find_perfect_reflection(&inputs[1], false);

        assert_eq!(reflection, Reflection::Horizontal(4));
    }

    #[test]
    fn finds_perfect_reflections() {
        let inputs = get_input();

        let total = part_1(&inputs);

        assert_eq!(total, 405);
    }

    #[test]
    fn finds_perfect_reflection_part_2() {
        let inputs = get_input();

        let reflection = find_perfect_reflection(&inputs[0], true);

        assert_eq!(reflection, Reflection::Horizontal(3));

        let reflection = find_perfect_reflection(&inputs[1], true);

        assert_eq!(reflection, Reflection::Horizontal(1));
    }

    #[test]
    fn finds_perfect_reflections_part_2() {
        let inputs = get_input();

        let total = part_2(&inputs);

        assert_eq!(total, 400);
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
