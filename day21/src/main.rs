use std::fs::File;
use std::io::{prelude::*, BufReader};
use num::Complex;
use std::collections::HashMap;

struct Map { map: Vec<Vec<char>>, start: Complex<i32> }

fn parser(path: &str) -> Map {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };

    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            line.expect("Line error")
                .chars()
                .collect()
        })
        .collect();

    lines.insert(0, vec!['@';lines[0].len()]);
    lines.push(vec!['@';lines[0].len()]);
    for i in 0..lines.len() {
        lines[i].insert(0, '@');
        lines[i].push('@');
    }

    let mut s: Complex<i32> = Complex::new(0, 1);

    let mut found = false;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i][j] == 'S' { 
                s = Complex::new(j as i32, i as i32);
                found = true;
                break;
            }
        }
        if found == true { break; }
    };

    let out = Map { map: lines, start: s };
    out
}

fn bfs(input: Map) -> usize {
    let mut visited = vec![];
    let mut queue = vec![];
    let mut current = input.start.clone();
    let directions = vec![
        Complex::new(1, 0),
        Complex::new(-1, 0),
        Complex::new(0, 1),
        Complex::new(0, -1),
    ];

    queue.push((current, 0));
    visited.push(current.clone());

    let mut ends = vec![];

    while let Some((current, steps)) = queue.pop() {
        let mut copy = input.map.clone();
        if steps == 6 {
            ends.push(current.clone());
        } else if steps > 6 {
            break;
        } else {
            for d in &directions {
                let check = current + d;
                if input.map[check.im as usize][check.re as usize] == '.' && !visited.contains(&check)
                {
                    queue.push((check, steps + 1));
                    visited.push(check);
                    copy[check.im as usize][check.re as usize] = 'O';
                }
            }
        }
        for c in copy { println!("{:?}", c) }
    }

    ends.len()
}

fn main() {
    let data = parser("./input");
    //for d in &data.map { println!("{:?}", d) }
    //println!("{}", &data.start);
    let part_1 = bfs(data);
    println!("{part_1}");
}
