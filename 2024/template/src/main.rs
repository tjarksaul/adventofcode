use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1();

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1() -> usize {
    0
}

fn part_2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        assert_eq!(0, 1);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<String> {
        i.lines()
            .map(|l| l.to_string())
            .collect()
    }
}
