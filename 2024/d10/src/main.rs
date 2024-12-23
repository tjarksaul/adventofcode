use pathfinding::prelude::{bfs_reach, count_paths};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2(&input);

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .enumerate()
        .map(move |(i, v)| {
            v.iter().enumerate().map(move |(j, x)| {
                if *x == 0 {
                    Some((i.clone(), j.clone()))
                } else {
                    None
                }
            })
        })
        .flatten()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|trailhead| {
            bfs_reach(trailhead, |pos| find_successors(&pos, &input)).collect::<Vec<_>>()
        })
        .flatten()
        .fold(0, |prev, (i, j)| {
            prev + if input[i][j] == 9 { 1 } else { 0 }
        })
}

fn part_2(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .enumerate()
        .map(move |(i, v)| {
            v.iter().enumerate().map(move |(j, x)| {
                if *x == 0 {
                    Some((i.clone(), j.clone()))
                } else {
                    None
                }
            })
        })
        .flatten()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|trailhead| {
            bfs_reach(trailhead, |pos| find_successors(&pos, &input))
                .collect::<Vec<_>>()
                .iter()
                .filter(|(i, j)| input[*i][*j] == 9)
                .map(|dst| {
                    count_paths(trailhead, |pos| find_successors(&pos, &input), |c| c == dst)
                })
                .fold(0, |prev, cur| prev + cur)
        })
        .fold(0, |prev, cur| prev + cur)
}

fn find_successors(pos: &(usize, usize), map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .map(|(i, j)| {
            let new_i = if i < 0 {
                pos.0.checked_sub(1)
            } else {
                Some(pos.0 + (i as usize))
            };
            let new_j = if j < 0 {
                pos.1.checked_sub(1)
            } else {
                Some(pos.1 + (j as usize))
            };

            if let (Some(i), Some(j)) = (new_i, new_j) {
                if i < map.len() && j < map[i].len() {
                    let abs_diff = map[i][j].abs_diff(map[pos.0][pos.1]);
                    if map[i][j] > map[pos.0][pos.1] && abs_diff <= 1 {
                        Some((i, j))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Vec<usize>> {
        vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ]
    }

    #[test]
    fn runs_part_1() {
        let input = get_input();

        let result = part_1(&input);

        assert_eq!(result, 36);
    }

    #[test]
    fn runs_part_2() {
        let input = get_input();

        let result = part_2(&input);

        assert_eq!(result, 81);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<Vec<usize>> {
        i.lines()
            .map(|l| {
                l.to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect()
    }
}
