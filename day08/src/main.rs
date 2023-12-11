use std::fs::File;
use std::io::{prelude::*, BufReader, self};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Document {
    left_right: String,
    nodes: Vec<Vec<String>>,
}

macro_rules! calc_lcm {
    ($a:expr, $b:expr, $g:expr) => {
        ($a * $b).abs() / $g
    };
}

fn parser(path: &str) -> Document {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    let reader = BufReader::new(file);
    let re = Regex::new(r"[,\=\(\)]").unwrap();

    let mut lines: Vec<Vec<String>> = reader
        .lines()
        .map(|line| {
            line.expect("Line error")
                .split_whitespace()
                .flat_map(|x| re.split(x))
                .filter(|&s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .collect();
    lines.retain(|l| l.len() != 0);
    let (path, locations) = lines.split_at(1);
    let doc = Document {
        left_right: path[0][0].clone(),
        nodes: locations.to_owned(),
    };
    doc
}

fn solve(data: Document, start: String, end: Vec<String>) -> i32 {
    let mut locations = HashMap::new();
    for n in data.nodes.clone() {
        locations.insert(n[0].clone(), vec![n[1].clone(), n[2].clone()]);
    }
    let mut counter = 0usize;
    let mut current = start;
    let mut i = 0usize;
    let directions: Vec<char> = data.left_right.chars().collect();
    while !end.contains(&current) {
        i = {
            if i > directions.len() - 1 {
                0
            } else {
                i
            }
        };
        match directions[i] {
            'L' => {
                current = locations.get(&current).unwrap()[0].clone();
                counter += 1;
                i += 1;
            },
            'R' => {
                current = locations.get(&current).unwrap()[1].clone();
                counter += 1;
                i += 1;
            }
            _ => panic!("Direction error")
        }
    }
    counter as i32
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a
    } else {
        return gcd(b, a % b)
    }
}

fn lcm(input: Vec<i64>) -> i64 {
    let mut running_total = 0;
    for i in 0..input.len() {
        if running_total == 0 {
            running_total += calc_lcm!(input[i], input[i+1], gcd(input[i], input[i+1]));
        } else if i == 1 {
            continue;
        } else {
            running_total = calc_lcm!(running_total, input[i], gcd(running_total, input[i]))
        }
    }
    running_total
}

fn part_1(data: Document) -> i32 {
    solve(data, "AAA".to_string(), vec!["ZZZ".to_string()])
}

fn part_2(data: Document) -> i64 {
    let (re_a, re_z) = (Regex::new(r"A$").unwrap(), Regex::new(r"Z$").unwrap());
    let mut a = vec![];
    let mut z = vec![];
    let mut values = vec![];
    for d in 0..data.nodes.len() {
        if re_a.is_match(&data.nodes[d][0]) {
            a.push(data.nodes[d][0].clone())
        } else if re_z.is_match(&data.nodes[d][0]) {
            z.push(data.nodes[d][0].clone())
        }
    }
    for x in a {
        values.push(solve(data.clone(), x, z.clone()) as i64);
    }
    lcm(values)
}

fn main() -> io::Result<()> {
    let data = parser("./input");
    println!("Part 1: {}", part_1(data.clone()));
    println!("Part 2: {}", part_2(data));
    
    Ok(())
}
