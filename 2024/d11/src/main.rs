use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2(&input);

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<usize>) -> usize {
    let mut stones = input.clone();

    for _ in 0..25 {
        let mut new_stones = vec![];

        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
                continue;
            }

            if let Some((left, right)) = split(stone) {
                new_stones.push(left);
                new_stones.push(right);
                continue;
            }

            new_stones.push(stone * 2024);
        }

        stones = new_stones;
    }

    stones.len()
}

fn part_2(input: &Vec<usize>) -> usize {
    let stones = input.clone();

    let mut memory: HashMap<(usize, usize), usize> = HashMap::new();
    let mut total_stones = 0;

    stones.into_iter().for_each(|num| {
        total_stones += compute_count(num, 75, &mut memory);
    });

    total_stones
}

fn split(stone: usize) -> Option<(usize, usize)> {
    let digits = (stone.checked_ilog10().unwrap_or(0)) + 1;
    if digits % 2 == 0 {
        let half = digits / 2;
        let pow = 10usize.pow(half);
        let left_half = stone / pow;
        let right_half = stone - (left_half * pow);

        return Some((left_half, right_half));
    }

    None
}

fn compute_count(
    num: usize,
    remaining_blinks: usize,
    memory: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if memory.contains_key(&(num, remaining_blinks)) {
        return memory[&(num, remaining_blinks)];
    }

    if remaining_blinks == 0 {
        return 1;
    }

    let remaining_blinks = remaining_blinks.checked_sub(1).unwrap_or(0);
    let result = if num == 0 {
        compute_count(1, remaining_blinks, memory)
    } else if let Some(parts) = split(num) {
        compute_count(parts.0, remaining_blinks, memory)
            + compute_count(parts.1, remaining_blinks, memory)
    } else {
        compute_count(num * 2024, remaining_blinks, memory)
    };

    memory.insert((num, remaining_blinks + 1), result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let input = vec![125, 17];

        let result = part_1(&input);

        assert_eq!(result, 55312);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<usize> {
        i.split_whitespace().map(|x| x.parse().unwrap()).collect()
    }
}
