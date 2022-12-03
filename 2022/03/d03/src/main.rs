mod part1;

fn main() {
    let input = part1::read::read_input("input.txt".to_string());
    let priorities = part1::calculate_priorities(input);

    println!("Sum of priorities: {}", priorities);
}
