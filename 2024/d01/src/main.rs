use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Vec<usize>>) -> usize {
    let len = input[0].len();
    let mut v1 = input[0].clone();
    v1.sort();
    let mut v2 = input[1].clone();
    v2.sort();

    let mut sum = 0;
    for i in 0..len {
        let diff = v1[i].abs_diff(v2[i]);
        sum += diff;
    }
    sum
}

fn part_2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let lists = vec![
            vec![3, 4, 2, 1, 3, 3],
            vec![4, 3, 5, 3, 9, 3],
        ];

        let result = part_1(&lists);

        assert_eq!(result, 11);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<Vec<usize>> {
        let mut v1: Vec<usize> = vec![];
        let mut v2: Vec<usize> = vec![];

        i.lines()
            .for_each(|l| {
                let (i1, i2) = l.split_once("   ").unwrap();
                v1.push(i1.parse().unwrap());
                v2.push(i2.parse().unwrap());
            });

        vec![v1, v2]
    }
}
