use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Mul>) -> usize {
    input.iter().fold(0, |prev, cur| prev + cur.run())
}

fn part_2() -> usize {
    0
}

#[derive(Debug, Clone, Copy)]
struct Mul(usize, usize);

impl Mul {
    fn run(self) -> usize {
        self.0 * self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();

        let data = read::read_all_lines(input);

        let result = part_1(&data);

        assert_eq!(161, result);
    }
}

mod read {
    use regex::Regex;
    use super::Mul;

    pub fn read_all_lines(i: String) -> Vec<Mul> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut results = vec![];
        for (_, [m1, m2]) in re.captures_iter(&i).map(|c| c.extract()) {
            results.push(Mul(m1.parse().unwrap(), m2.parse().unwrap()));
        }

        results
    }
}
