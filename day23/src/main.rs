use std::fs::File;
use std::io::{prelude::*, BufReader};
use num::complex::Complex;
use std::collections::HashMap;
use std::cmp::max;

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<char>>,
    start: Complex<i32>,
    end: Complex<i32>
}

fn parser(path: &str) -> Map {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };

    let reader = BufReader::new(file);

    let mut s: Complex<i32> = Complex::new(0, 0);
    let mut e: Complex<i32> = Complex::new(0, 0);

    let mut lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            line.expect("Line error")
                .chars()
                .collect()
        }).collect();

    lines.insert(0, vec!['#'; lines[0].len()]);

    for (i,&v) in lines[1].iter().enumerate() {
        if v == '.' { 
            s = Complex::new(i as i32, 1);
            break;
        }
    }

    for i in 0..lines[0].len() {
        if lines[lines.len()-1][i] == '.' {
            e = Complex::new(i as i32, (lines.len()-1) as i32);
        }
    }

    lines.push(vec!['#'; lines[0].len()]);

    let out = Map { tiles: lines, start: s, end: e };
    out
}

fn traverse(tiles: &[Vec<char>], s: &Complex<i32>, e: &Complex<i32>) {
    let mut queue: Vec<Complex<i32>> = vec![];
    let mut visited: Vec<Complex<i32>> = vec![];
    let mut current = Complex::new(0,0);
    let directions: Vec<Complex<i32>> = vec![
        Complex::new(1, 0), Complex::new(0, 1), Complex::new(-1, 0), Complex::new(0, -1), 
    ];
    let mut parents = HashMap::new();
    //let mut copy = tiles.clone().to_owned();
    let mut prev = 'x';
    let mut max_path = 0;

    visited.push(*s);
    queue.push(*s);

    while queue.len() != 0 {
        prev = tiles[current.im as usize][current.re as usize];
        current = queue.pop().unwrap();

        if current == *e { 
            max_path = max(max_path, parents.len());
            continue;
        }

        let curr = tiles[current.im as usize][current.re as usize];
        
        for d in &directions {
            let check = current + d;
            let next = tiles[check.im as usize][check.re as usize];

            if !visited.contains(&check) {
                match curr {
                    '.' => {
                        if next != '#' {
                            visited.push(check);
                            parents.insert(check, current);
                            queue.push(check);
                        }
                    },
                    '>'|'<'|'^'|'v' => {
                        if vec!['>','<','^','v'].contains(&prev) && next == '.' {
                            visited.push(check);
                            parents.insert(check, current);
                            queue.push(check);
                        } else if prev == '.' && next == '.' {
                            match (curr, d.re, d.im) {
                                ('>',1,0) | ('<',-1,0) | ('^',0,-1) | ('v',0,1) => {
                                    visited.push(check);
                                    parents.insert(check, current);
                                    queue.push(check);
                                },
                                _ => continue
                            }
                        } else {
                            match (next, d.re, d.im) {
                                ('>',1,0) | ('<',-1,0) | ('^',0,-1) | ('v',0,1) => {
                                    visited.push(check);
                                    parents.insert(check, current);
                                    queue.push(check);
                                },
                                _ => continue
                            }
                        }
                    },
                    _ => continue
                }
            }
        }
    }

    /*let mut v = &e.clone();
    copy[e.im as usize][e.re as usize] = 'E';
    copy[s.im as usize][s.re as usize] = 'S';

    while v != s {
        println!("{}",copy[v.im as usize][v.re as usize]);
        v = parents.get(v).unwrap();
        if v != e && !vec!['>','<','^','v'].contains(&copy[v.im as usize][v.re as usize] ) {
            copy[v.im as usize][v.re as usize] = '_' 
        }
    }

    for c in copy { println!("{:?}", c.iter().collect::<String>()) }*/
    println!("{max_path}");
}

fn main() {
    let data = parser("./input");
    traverse(&data.tiles, &data.start, &data.end);
    println!("done")
}
