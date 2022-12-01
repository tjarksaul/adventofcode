mod read;

fn main() {
    let input = read::read_input("input.txt".to_string());
    let calories = calculate_calories(input);
    let input_2 = read::read_input("input.txt".to_string());
    let top_three_calories = calculate_top_three_calories(input_2);

    println!("Max number of calories: {}", calories);
    println!("Sum of top three calories: {}", top_three_calories);
 }

 fn get_sums(vec: Vec<Vec<i32>>) -> Vec<i32> {
    return vec.iter().map(|v| v.iter().fold(0, |a, b| a + b)).collect();
 }

 pub fn calculate_calories(vec: Vec<Vec<i32>>) -> i32 {
    let sums = get_sums(vec);

    let max_v = sums.iter().max();
    match max_v {
        Some(max) => return *max,
        None      => return 0,
    }    
 }

 pub fn calculate_top_three_calories(vec: Vec<Vec<i32>>) -> i32 {
    let mut sums = get_sums(vec);
    sums.sort_unstable_by(|a, b| b.cmp(a));
    let top_three = sums[0..3].to_vec();

    return top_three.iter().fold(0, |a, b| a + b);
 }

#[cfg(test)]
mod tests {
    fn get_test_input() -> Vec<Vec<i32>> {
        return vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
    }

    #[test]
    fn calculates_calories_correctly() {
        let calories = super::calculate_calories(get_test_input());

        assert_eq!(calories, 24000);
    }

    #[test]
    fn calculates_top_three_calories_correctly() {
        let calories = super::calculate_top_three_calories(get_test_input());

        assert_eq!(calories, 45000);
    }
}
