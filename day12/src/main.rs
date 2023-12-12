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

/*
    Initially I tried to run this funciton with the raw string as input including the ?'s and check
    all perutations that way. It worked for simple cases but I couldn't work out a way to handle
    future '?' values when the '#' branch looks forward to check for groups. It probably could have
    worked, but I couldn't fix it, especially with how badly written it is.
*/
fn solver(curr: String, cond: Vec<String>) -> i32 {
    let mut new_string = "".to_string();
    let mut curr_chars: Vec<char> = curr.chars().collect();
    let mut new_conds = cond.clone();
    
    if curr_chars.len() == 0 && new_conds.len() == 0 {
        return 1;
    } else if curr_chars.len() == 0 && new_conds.len() != 0 {
        return 0;
    } else if curr_chars.len() != 0 && new_conds.len() == 0 {
        if curr_chars.contains(&'#') {
            return 0;
        } else {
            return 1;
        }
    }
    
    match curr_chars[0] {
        '.' => {
            curr_chars.remove(0);
            new_string = curr_chars.iter().collect();
            solver(new_string, new_conds)
        },
        '#' => {
            let group_n = cond[0].parse::<i32>().unwrap();
            let mut count = 0;
            if group_n > (curr_chars.len() as i32) {
                return 0;
            };
            let (mut dot, mut i) = (false, 0 as usize);
            while !dot && i < curr_chars.len() {
                if curr_chars[i] == '#' { count += 1 }
                else { dot = true }
                i += 1
            }
            curr_chars.drain(0..(count as usize));
            new_string = curr_chars.iter().collect();
            if count == group_n {
                new_conds.remove(0);
                solver(new_string, new_conds)
            } else {
                return 0;
            }
        },
            _ => panic!("Invalid Character")
    }
}

fn permutations(s: &str, current: &mut String, index: usize, result: &mut Vec<String>) {
    if index == s.len() {
        result.push(current.clone());
        return;
    }

    let c = s.chars().nth(index).unwrap();

    if c == '?' {
        current.push('.');
        permutations(s, current, index + 1, result);
        current.pop();

        current.push('#');
        permutations(s, current, index + 1, result);
        current.pop();
    } else {
        current.push(c);
        permutations(s, current, index + 1, result);
        current.pop();
    }
}

fn part_1(data: Data) -> i32 {
    let mut result = 0;
    for i in 0..data.strings.len() {
        let mut perms = vec![];
        permutations(&data.strings[i as usize], &mut "".to_string(), 0, &mut perms);
        for p in perms {
            let valid = solver(p.clone(), data.conditions[i as usize].clone());
            result += valid;
        }
    }
    result
}

fn main() {
    let data = parser("./input");
    println!("Part 1: {}", part_1(data));
}
