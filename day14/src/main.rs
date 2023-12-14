use std::fs::File;
use std::io::{prelude::*, BufReader, self};

fn parser(path: &str) -> Vec<Vec<char>> {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };

    let reader = BufReader::new(file);

    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            line.expect("Line error")
                .chars()
                .into_iter().map(|x| x.to_owned())
                .collect()
        })
        .collect();
    lines
}

fn north(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut modified = input.clone();
    
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 'O' {
                for k in (0..i).rev() {
                    match modified[k][j] {
                        '.' => {
                            if k == 0 {
                                modified[i][j] = '.';
                                modified[0][j] = 'O';
                                break
                            } else { continue }
                        },
                        '#' | 'O' => {
                            modified[i][j] = '.';
                            modified[k+1][j] = 'O';
                            break
                        },
                        _ => panic!("Invalid character"),
                    }
                }
            }
        }
    }

    modified
}

fn rotate(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut rotated = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[j][rows - 1 - i] = matrix[i][j];
        }
    }

    rotated
}

fn calc_load(input: Vec<Vec<char>>, dir: &str) -> (i64, Vec<Vec<char>>) {
    let mut load: i64 = 0;
    
    let modified = match dir {
        "north" => north(input.clone()),
        "cycle" => {
            let mut result = input.clone();
            
            for i in 0..4 {
                result = north(result);
                result = rotate(result);
            }
            
            result
        },
        _ => panic!("Invalid dir: dir = north or cycle")
    };
    
    for i in 0..modified.len() {
        for j in 0..modified[0].len() {
            if modified[i][j] == 'O' {
                load += (modified.len() - i) as i64
            }
        }
    }
    (load, modified)
}

fn main() {
    let data = parser("./input");
    let part_1 = calc_load(data.clone(), "north");
    println!("Part 1: {}", part_1.0)
}
