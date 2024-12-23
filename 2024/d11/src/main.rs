use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

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

            let digits = (stone.checked_ilog10().unwrap_or(0)) + 1;
            if digits % 2 == 0 {
                let half = digits / 2;
                let pow = 10usize.pow(half);
                let left_half = stone / pow;
                let right_half = stone - (left_half * pow);

                new_stones.push(left_half);
                new_stones.push(right_half);
                continue;
            }

            new_stones.push(stone * 2024);
        }

        stones = new_stones;
    }

    stones.len()
}

fn part_2() -> usize {
    0
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
        i.split(" ").map(|x| x.parse().unwrap()).collect()
    }
}
