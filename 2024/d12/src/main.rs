use std::collections::{HashSet, VecDeque};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();
    let mut regions = vec![];
    let height = input.len();
    let width = input[0].len();
    let size = height * width;
    let mut pos = (0, 0);

    while visited.len() < size {
        let chr = input[pos.0][pos.1];

        let mut nodes = vec![];
        let mut queue = VecDeque::from([pos]);

        while let Some(node) = queue.pop_front() {
            if chr == input[node.0][node.1] {
                nodes.push(node);
                visited.insert(node);

                if node.0 > 0 {
                    let new_node = (node.0 - 1, node.1);
                    if !nodes.contains(&new_node) && !queue.contains(&new_node) {
                        queue.push_back(new_node);
                    }
                }
                if node.1 > 0 {
                    let new_node = (node.0, node.1 - 1);
                    if !nodes.contains(&new_node) && !queue.contains(&new_node) {
                        queue.push_back(new_node);
                    }
                }
                if node.0 < input.len() - 1 {
                    let new_node = (node.0 + 1, node.1);
                    if !nodes.contains(&new_node) && !queue.contains(&new_node) {
                        queue.push_back(new_node);
                    }
                }
                if node.1 < input[node.0].len() - 1 {
                    let new_node = (node.0, node.1 + 1);
                    if !nodes.contains(&new_node) && !queue.contains(&new_node) {
                        queue.push_back(new_node);
                    }
                }
            }
        }

        regions.push(nodes.clone());

        // let's find the next position to start filling from
        'outer: for i in 0..input.len() {
            for j in 0..input.len() {
                if !visited.contains(&(i, j)) {
                    pos = (i, j);
                    break 'outer;
                }
            }
        }
    }

    regions
        .iter()
        .map(|region| {
            let area = region.len();
            let perimeter = region.iter().fold(0, |prev, (i, j)| {
                // outside ones are perimeter
                prev
                    + if *i == 0 || *i == (height - 1) { 1 } else { 0 }
                    + if *j == 0 || *j == (width - 1) { 1 } else { 0 }
                    // now for the ones where neighbors are different
                    + if *i > 0 && input[*i - 1][*j] != input[*i][*j] { 1 } else { 0 }
                    + if *i < height - 1 && input[*i + 1][*j] != input[*i][*j] { 1 } else { 0 }
                    + if *j > 0 && input[*i][*j - 1] != input[*i][*j] { 1 } else { 0 }
                    + if *j < width - 1 && input[*i][*j + 1] != input[*i][*j] { 1 } else { 0 }
            });
            (area, perimeter)
        })
        .fold(0, |prev, (area, perimeter)| prev + (area * perimeter))
}

fn part_2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let s = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            .to_string();

        let input = read::read_all_lines(s);

        let result = part_1(&input);

        assert_eq!(result, 1930);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<Vec<char>> {
        i.lines().map(|l| l.to_string().chars().collect()).collect()
    }
}
