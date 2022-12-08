fn main() {
    let input = read::read_input(String::from("input.txt"));

    let visible_tree_count = count_visible_trees(input);

    println!("Found {visible_tree_count} visible trees.");
}

fn count_visible_trees(trees: Vec<Vec<i32>>) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_correctly() {
        let trees = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        let visible_tree_count = count_visible_trees(trees);

        assert_eq!(visible_tree_count, 21);
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
