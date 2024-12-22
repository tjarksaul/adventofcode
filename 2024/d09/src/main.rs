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
    let mut typ = 0;
    let mut id = 0;
    let mut disk = vec![];

    for val in input {
        if typ == 0 {
            // we have a file
            for _ in 0..*val {
                disk.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..*val {
                disk.push(None);
            }
        }

        typ = (typ + 1) % 2;
    }

    // write_disk(&disk);

    // now let's frag!

    'outer: loop {
        // find rightmost value
        let mut pos = disk.len();

        for i in (0..disk.len()).rev() {
            if disk[i].is_some() {
                pos = i;
                break;
            }
        }

        if pos == disk.len() {
            panic!("We should never be here!");
        }

        // now we need to find the first empty position
        for i in 0..disk.len() {
            if i == pos {
                // we arrived at the original position of this item so we can't change anything anymore, we're done!
                break 'outer;
            }

            if disk[i].is_none() {
                // we found an empty position where we can move stuff!
                disk[i] = disk[pos];
                disk[pos] = None;
                break;
            }
        }
    }

    // write_disk(&disk);

    disk.iter()
        .enumerate()
        .map(|(i, v)| match v {
            Some(x) => x * i,
            None => 0,
        })
        .fold(0, |prev, cur| prev + cur)
}

#[allow(dead_code)]
fn write_disk(disk: &Vec<Option<usize>>) {
    for val in disk {
        if let Some(x) = val {
            print!("{x}");
        } else {
            print!(".");
        }
    }
    println!("");
}

fn part_2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let s = "2333133121414131402".to_string();

        let input = read::read_all_lines(s);

        let result = part_1(&input);

        assert_eq!(result, 1928);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<usize> {
        i.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}
