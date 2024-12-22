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
    let mut disk = map_disk(&input);

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

    calculate_checksum(&disk)
}

fn part_2(input: &Vec<usize>) -> usize {
    let mut disk = map_disk_2(&input);
    let max = disk
        .iter()
        .map(|(_, x)| x)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .max()
        .unwrap();

    // write_disk(&disk);

    for id_no in (0..max + 1).rev() {
        // find rightmost value
        let mut pos = disk.len();
        let mut len = 0;

        for i in 0..disk.len() {
            let (length, val) = disk[i];
            if let Some(x) = val {
                if x == id_no {
                    // we found the file
                    pos = i;
                    len = length;
                    break;
                }
            }
        }

        if pos == disk.len() {
            panic!("We should never be here!");
        }

        // now we need to find the first empty position
        for new_pos in 0..pos {
            if disk[new_pos].1.is_none() {
                // we found an empty position

                if disk[new_pos].0 >= len {
                    // there's enough space, so we can move the whole file
                    let to_move = disk[pos];

                    // remove element from disk and increase free space to the left (or right if that's the only free space) by its size
                    if pos > 0 && disk[pos - 1].1.is_none() {
                        // found free space to the left
                        let amount = disk[pos - 1].0;
                        disk[pos - 1].0 = amount + to_move.0;
                        disk.remove(pos);
                    } else if pos < disk.len() - 1 && disk[pos + 1].1.is_none() {
                        // found free space to the right
                        let amount = disk[pos + 1].0;
                        disk[pos + 1].0 = amount + to_move.0;
                        disk.remove(pos);
                    } else {
                        disk[pos] = (to_move.0, None);
                    }

                    let remaining_free_space = disk[new_pos].0 - len;
                    disk[new_pos] = to_move;
                    if remaining_free_space > 0 {
                        disk.insert(new_pos + 1, (remaining_free_space, None));
                    }
                    break;
                }
            }
        }

        // write_disk(&disk);
    }

    // write_disk(&disk);

    calculate_checksum_2(&disk)
}

fn map_disk(input: &Vec<usize>) -> Vec<Option<usize>> {
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

    disk
}

fn map_disk_2(input: &Vec<usize>) -> Vec<(usize, Option<usize>)> {
    let mut typ = 0;
    let mut id = 0;
    let mut disk = vec![];

    for val in input {
        if typ == 0 {
            // we have a file
            disk.push((*val, Some(id)));
            id += 1;
        } else {
            disk.push((*val, None));
        }

        typ = (typ + 1) % 2;
    }

    disk
}

fn calculate_checksum(disk: &Vec<Option<usize>>) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, v)| match v {
            Some(x) => x * i,
            None => 0,
        })
        .fold(0, |prev, cur| prev + cur)
}

fn calculate_checksum_2(disk: &Vec<(usize, Option<usize>)>) -> usize {
    disk.iter()
        .fold((0, 0), |(pos, sum), (len, val)| {
            println!("({pos}, {sum}), ({len}, {})", val.unwrap_or(999999));
            match val {
                Some(id) => (
                    pos + len,
                    sum + (pos..(pos + len)).fold(0, |prev, cur| prev + cur * id),
                ),
                None => (pos + len, sum),
            }
        })
        .1
}

#[allow(dead_code)]
fn write_disk(disk: &Vec<(usize, Option<usize>)>) {
    for (len, val) in disk {
        for _ in 0..*len {
            if let Some(x) = val {
                print!("{x}");
            } else {
                print!(".");
            }
        }
    }
    println!("");
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

    #[test]
    fn runs_part_2() {
        let s = "2333133121414131402".to_string();

        let input = read::read_all_lines(s);

        let result = part_2(&input);

        assert_eq!(result, 2858);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<usize> {
        i.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}
