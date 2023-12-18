use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec;
use num::complex::Complex;
use std::collections::HashSet;

fn parser(path: &str) -> Vec<Vec<char>> {
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
                .into_iter().map(|x| x.to_owned())
                .collect()
        })
        .collect();
    
    lines.insert(0, vec!['@';lines[0].len()]);
    lines.push(vec!['@';lines[0].len()]);
    for i in 0..lines.len() {
        lines[i].insert(0, '@');
        lines[i].push('@');
    }
    
    lines
}

fn search(input: &[Vec<char>], v: &(i32, i32), dir: Complex<i32>) -> (i32, Vec<Complex<i32>>) {
    let mut current = Complex::new(v.0, v.1);
    let mut queue: Vec<(Complex<i32>,Complex<i32>)> = vec![];
    let mut visited: Vec<Complex<i32>> = vec![];
    let mut direction = dir;
    let mut energized = 0;
    let mut cache: HashSet<(Complex<i32>,Complex<i32>)> = HashSet::new();
    
    queue.push((current, direction));
    
    while queue.len() != 0 {
        let vals = queue.pop().unwrap();

        if cache.contains(&vals) || cache.contains(&(vals.0, Complex::new(-1,-1) * vals.1)) {
            continue;
        } else {
            cache.insert(vals);
        }

        current = vals.0;
        direction = vals.1;

        if !visited.contains(&current) {
            energized += 1;
            visited.push(current);
        }

        //println!("{} : {}", current, direction);
        
        match input[current.im as usize][current.re as usize] {
            '/' => {
                match (direction.re, direction.im) {
                    (1, 0) => {
                        direction = Complex::new(0, -1);
                    },
                    (-1, 0) => {
                        direction = Complex::new(0, 1);
                    },
                    (0, 1) => {
                        direction = Complex::new(-1, 0);
                    },
                    (0, -1) => {
                        direction = Complex::new(1, 0);
                    },
                    _ => panic!("Invalid direction"),
                }
                queue.push((current + direction, direction));
            },
            '\\' => {
                match (direction.re, direction.im) {
                    (1,0) => {
                        direction = Complex::new(0, 1);
                    },
                    (-1,0) => {
                        direction = Complex::new(0, -1);
                    },
                    (0,1) => {
                        direction = Complex::new(1, 0);
                    },
                    (0,-1) => {
                        direction = Complex::new(-1, 0);
                    },
                    _ => panic!("Invalid direction"),
                }
                queue.push((current + direction, direction));
            },
            '.' => {
                queue.push((current + direction, direction));
            },
            '-' => {
                match (direction.re, direction.im) {
                    (0,1) | (0,-1) => {
                        queue.push((current + Complex::new(1,0), Complex::new(1,0)));
                        queue.push((current + Complex::new(-1,0), Complex::new(-1,0)));
                    },
                    (1,0) | (-1,0) => {
                        queue.push((current + direction, direction));
                    },
                    _ => panic!("Invalid direction"),
                }
            },
            '|' => {
                match (direction.re, direction.im) {
                    (1,0) | (-1,0) => {
                        queue.push((current + Complex::new(0,1), Complex::new(0,1)));
                        queue.push((current + Complex::new(0,-1), Complex::new(0,-1)));
                    },
                    (0,1) | (0,-1) => {
                        queue.push((current + direction, direction));
                    },
                    _ => panic!("Invalid direction"),
                }
            },
            '@' => energized -= 1,
            _ => panic!("Invalid character")
        }
    }
    (energized,visited)
}

fn main() {
    let data = parser("./input");
    let result = search(&data, &(1,1), Complex::new(1, 0)).0;
    println!("Part 1: {result}");

    let mut results =  vec![];
    let mut starts = vec![];
    let (j, k, l) = (1 as usize, data[0].len()-2, data.len()-2);

    for i in 1..data.len()-1 {
        if i == 1 {
            starts.push(((i as i32, j as i32), Complex::new(1, 0)));
            starts.push(((i as i32, k as i32), Complex::new(-1, 0)));
            starts.push(((i as i32, j as i32), Complex::new(0, 1)));
            starts.push(((i as i32, k as i32), Complex::new(0, -1)));
        } else {
            starts.push(((i as i32, j as i32), Complex::new(0, 1)));
            starts.push(((i as i32, k as i32), Complex::new(0, -1)));
        }
    }

    for i in 1..data[0].len()-1 {
        starts.push(((j as i32, i as i32), Complex::new(1, 0)));
        starts.push(((l as i32, i as i32), Complex::new(-1, 0)));
    }

    for s in starts.clone() {
        results.push(search(&data, &(s.0.0, s.0.1), s.1).0)
    }

    println!("Part 2: {}", results.iter().max().unwrap());
}
