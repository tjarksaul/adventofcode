mod read;

fn main() {
    let input = read::read_input("input.txt".to_string());
    let calories = calculate_calories(input);

    println!("Max number of calories: {}", calories)
 }

 fn calculate_calories(vec: Vec<Vec<i32>>) -> i32 {
    let sums: Vec<_> = vec.iter().map(|v| v.iter().fold(0, |a, b| a + b)).collect();

    let max_v = sums.iter().max();
    match max_v {
        Some(max) => return *max,
        None      => return 0,
    }    
 }

// #[cfg(test)]
// mod tests {
    #[test]
    fn calculates_calories_correctly() {
        let mut vec = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        let calories = calculate_calories(vec);

        assert_eq!(calories, 24000);
    }
// }
