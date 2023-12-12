use std::fs::File;
use std::io::{prelude::*, BufReader, self};

#[derive(Debug, Clone)]
struct Data {
    strings: Vec<String>,
    conditions: Vec<Vec<String>>,
}

fn parser(path: &str) -> Data {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    let reader = BufReader::new(file);
    let lines: Vec<Vec<String>> = reader
        .lines()
        .map(|line| {
            line.expect("Line error")
                .split_whitespace()
                .into_iter().map(|x| x.to_owned())
                .collect()
        })
        .collect();
    let (mut zero, mut one) = (vec![],vec![]);
    for l in 0..lines.len() {
        zero.push(lines[l][0].clone());
        one.push(lines[l][1].split(',').into_iter().map(|x| x.to_owned()).collect());
    }
    let parsed = Data {
        strings: zero,
        conditions: one
    };
    parsed
}    

fn solver(curr: String, cond: Vec<String>, p: i64, prev: (Option<char>, bool)) -> i64 {
    let mut new_string = "".to_string();
    let mut curr_chars: Vec<char> = curr.chars().collect();
    let mut new_conds = cond.clone();
    let mut permutations: i64 = p;
    
    println!("{:?} {:?}", curr_chars, new_conds);
    
    if curr_chars.len() == 0 && new_conds.len() == 0 {
        permutations += 1;
        println!("Valid");
        return permutations;
    } else if curr_chars.len() == 0 && new_conds.len() != 0 {
        println!("Failed");
        return permutations;
    } else if new_conds.len() == 0 {
        permutations += 1;
        return permutations;
    }
    
    match curr_chars[0] {
        '.' => {
            curr_chars.remove(0);
            new_string = curr_chars.iter().collect();
            solver(new_string, new_conds, permutations, (None, false))
        },
        '#' => {
            let group_n = cond[0].parse::<i32>().unwrap();
            let mut count = 0;
            if group_n > (curr_chars.len() as i32) {
                return permutations;
            };
            for i in 0..group_n {
                if curr_chars[i as usize] != '.' { count += 1 }
            }
            curr_chars.drain(0..(count as usize));
            new_string = curr_chars.iter().collect();
            if count == group_n {
                new_conds.remove(0);
                solver(new_string, new_conds, permutations, (Some('#'), true))
            } else {
                solver(new_string, new_conds, permutations, (Some('#'), false))
            }
        },
        '?' => {
            curr_chars[0] = '.';
            new_string = curr_chars.iter().collect();
            let result1 = solver(new_string, new_conds.clone(), permutations, (None, false));

            curr_chars[0] = '#';
            new_string = curr_chars.iter().collect();
            let result2 = if prev.0 == Some('#') && prev.1 == true {
                0
            } else { solver(new_string, new_conds.clone(), permutations, (Some('#'), false)) };
            
            // Accumulate results, but don't directly add to permutations here
            result1 + result2
        }
            _ => panic!("Invalid Character")
    }
}

fn part_1(data: Data) {
    let string = data.strings[0].clone();
    let cond = data.conditions[0].clone();
    println!("{}", solver(string, cond, 0, (None, false)));
}

fn main() {
    let data = parser("./input");
    part_1(data)
}
