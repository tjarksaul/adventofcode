use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_input(fname: String) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = Vec::new();
    let mut current: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(fname) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    vec.push(current);
                    current = Vec::new();
                } else {
                    current.push(ip.parse::<>().unwrap());
                }
            }
        }
    }
    vec.push(current);
    return vec;
 }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
