fn main() {
    let input = read::read_input(String::from("input.txt"));

    let visible_tree_count = count_visible_trees(&input);
    let highest_scenic_distance = find_highest_scenic_distance(&input);

    println!("Found {visible_tree_count} visible trees.");
    println!("The highest scenic distance is {highest_scenic_distance}.");
}

fn count_visible_trees(trees: &Vec<Vec<i32>>) -> i32 {
    let mut visibility = vec![vec![false; trees[0].len()]; trees.len()];

    let mut column_heights = vec![-1; trees.len()];

    for row in 0..trees.len() {
        let mut row_height = -1;
        for column in 0..trees[row].len() {
            let tree = trees[row][column];
            if tree > row_height || tree > column_heights[column] {
                visibility[row][column] = true;
                if tree > row_height {
                    row_height = tree;
                }
                if tree > column_heights[column] {
                    column_heights[column] = tree;
                }
            }
        }
    }

    column_heights = vec![-1; trees.len()];

    for row in (0..trees.len()).rev() {
        let mut row_height = -1;
        for column in (0..trees[row].len()).rev() {
            let tree = trees[row][column];
            if tree > row_height || tree > column_heights[column] {
                visibility[row][column] = true;
                if tree > row_height {
                    row_height = tree;
                }
                if tree > column_heights[column] {
                    column_heights[column] = tree;
                }
            }
        }
    }

    return visibility
        .iter()
        .flat_map(|a| a)
        .fold(0, |a, b| a + *b as i32);
}

fn find_highest_scenic_distance(trees: &Vec<Vec<i32>>) -> i32 {
    let mut highest_scenic_distance = 0;

    let rows = trees.len();
    let columns = trees[0].len();

    for row in 0..rows {
        for column in 0..columns {
            let tree = trees[row][column];
            let mut distance_left = 0;
            for neigh_row in (0..row).rev() {
                distance_left += 1;
                if trees[neigh_row][column] >= tree {
                    break;
                }
            }
            if distance_left == 0 {
                continue;
            }

            let mut distance_right = 0;
            for neigh_row in (row + 1)..rows {
                distance_right += 1;
                if trees[neigh_row][column] >= tree {
                    break;
                }
            }
            if distance_right == 0 {
                continue;
            }

            let mut distance_top = 0;
            for neigh_col in (0..column).rev() {
                distance_top += 1;
                if trees[row][neigh_col] >= tree {
                    break;
                }
            }
            if distance_top == 0 {
                continue;
            }

            let mut distance_bottom = 0;
            for neigh_col in (column + 1)..columns {
                distance_bottom += 1;
                if trees[row][neigh_col] >= tree {
                    break;
                }
            }
            if distance_bottom == 0 {
                continue;
            }

            let scenic_distance = distance_left * distance_right * distance_bottom * distance_top;

            if scenic_distance > highest_scenic_distance {
                highest_scenic_distance = scenic_distance;
            }
        }
    }

    return highest_scenic_distance;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Vec<i32>> {
        return vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
    }

    #[test]
    fn test_counts_correctly() {
        let trees = get_input();

        let visible_tree_count = count_visible_trees(&trees);

        assert_eq!(visible_tree_count, 21);
    }

    #[test]
    fn finds_highest_scenic_distance_correctly() {
        let trees = get_input();

        let highest_scenic_distance = find_highest_scenic_distance(&trees);

        assert_eq!(highest_scenic_distance, 8);
    }
}

mod read {
    use std::fs;

    pub fn read_input(fname: String) -> Vec<Vec<i32>> {
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

        let splits: Vec<&str> = contents.lines().collect();

        return splits
            .iter()
            .map(|split| {
                split
                    .chars()
                    .map(|chr| chr.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect();
    }
}
