use std::collections::HashMap;

fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let shortest_path = find_shortest_path(&input.0, &input.1);
    let shortest_parallel_path = find_shortest_parallel_path(&input.0, &input.1);

    dbg!(shortest_path, shortest_parallel_path);
}

fn find_shortest_path(nodes: &HashMap<String, (String, String)>, directions: &Vec<u8>) -> usize {
    let mut length = 0;
    let mut cur = "AAA";

    loop {
        let direction = directions[length % directions.len()];
        if direction == b'R' {
            cur = &nodes.get(cur).unwrap().1;
        } else {
            cur = &nodes.get(cur).unwrap().0;
        }
        length +=1;
        if cur == "ZZZ" {
            break;
        }
    }

    length
}

fn find_shortest_parallel_path(nodes: &HashMap<String, (String, String)>, directions: &Vec<u8>) -> usize {
    let mut cur = vec![];

    // Find all starting positions
    for (node, _) in nodes {
        if node.chars().last().unwrap() == 'A' {
            cur.push(node);
        }
    }
    
    let mut results = HashMap::new();
    let mut length = 0;
    loop {
        let direction = directions[length % directions.len()];
        let mut new = vec![];
        for (i, node) in cur.iter().enumerate() {
            if direction == b'R' {
                new.push(&nodes.get(&node as &str).unwrap().1);
            } else {
                new.push(&nodes.get(&node as &str).unwrap().0);
            }

            let new_node = new.last().cloned().unwrap();

            if new_node.chars().last().unwrap() == 'Z' {
                results.insert(i, length + 1);
                // if we found all final lengths, the total lenths is the least common multiplier of all
                if results.len() == cur.len() {
                    return lcm(results.values().cloned().collect());
                }
            }    
        }
        cur = new;
        length +=1;
    }
}

fn lcm(xs: Vec<usize>) -> usize {
    xs.iter().fold(1, |acc, cur| (cur * acc) / gcd(*cur, acc))
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_way_correctly() {
        let inputs = vec![
            (
                vec![b'R', b'L'],
                HashMap::from([
                    ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string())),
                    ("BBB".to_string(), ("DDD".to_string(), "EEE".to_string())),
                    ("CCC".to_string(), ("ZZZ".to_string(), "GGG".to_string())),
                    ("DDD".to_string(), ("DDD".to_string(), "DDD".to_string())),
                    ("EEE".to_string(), ("EEE".to_string(), "EEE".to_string())),
                    ("GGG".to_string(), ("GGG".to_string(), "GGG".to_string())),
                    ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
                ]),
                2,
            ),
            (
                vec![b'L', b'L', b'R'],
                HashMap::from([
                    ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
                    ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
                    ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
                ]),
                6,
            )
        ];

        for (directions, nodes, length) in inputs.iter() {
            let path_length = find_shortest_path(&nodes, &directions);

            assert_eq!(path_length, *length);
        }
    }

    #[test]
    fn finds_parallel_way_correctly() {
        let directions = vec![b'L', b'R'];
        let nodes = HashMap::from([
            ("11A".to_string(), (("11B".to_string(), "XXX".to_string()))),
            ("11B".to_string(), (("XXX".to_string(), "11Z".to_string()))),
            ("11Z".to_string(), (("11B".to_string(), "XXX".to_string()))),
            ("22A".to_string(), (("22B".to_string(), "XXX".to_string()))),
            ("22B".to_string(), (("22C".to_string(), "22C".to_string()))),
            ("22C".to_string(), (("22Z".to_string(), "22Z".to_string()))),
            ("22Z".to_string(), (("22B".to_string(), "22B".to_string()))),
            ("XXX".to_string(), (("XXX".to_string(), "XXX".to_string()))),
        ]);

        let path_length = find_shortest_parallel_path(&nodes, &directions);

        assert_eq!(path_length, 6);
    }
}

mod read {
    use nom::{
        bytes::complete::{tag, take}, character::complete as cc, combinator::all_consuming,
        multi::{many0, separated_list1}, sequence::{terminated, tuple}, Finish, IResult,
    };
    use std::collections::HashMap;

    // Sample input
    // LLR

    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    pub fn read_all_lines(i: &'static str) -> (HashMap<String, (String, String)>, Vec<u8>) {
        all_consuming(parse_input)(i).finish().unwrap().1   
    }

    fn parse_input(i: &str) -> IResult<&str, (HashMap<String, (String, String)>, Vec<u8>)> {
        let (i, directions) = terminated(many0(parse_direction), tuple((cc::newline, cc::newline)))(i)?;

        let (i, nodes) = separated_list1(cc::newline, parse_node)(i)?;

        Ok((i, (
            nodes.iter().map(
                |s| (s.0.to_string(), (s.1.0.to_string(), s.1.1.to_string()))
            ).collect::<HashMap<_, _>>(), 
            directions
        )))
    }

    fn parse_direction(i: &str) -> IResult<&str, u8> {
        let (i, dir) = cc::one_of("RL")(i)?;

        Ok((i, dir as u8))
    }

    // AAA = (BBB, BBB)
    fn parse_node(i: &str) -> IResult<&str, (&str, (&str, &str))> {
        let (i, (src, _, left, _, right)) = terminated(tuple((
            take(3usize), 
            tag(" = ("), 
            take(3usize), 
            tag(", "),
            take(3usize), 
        )), tag(")"))(i)?;

        Ok((i, (src, (left, right))))
    }
}