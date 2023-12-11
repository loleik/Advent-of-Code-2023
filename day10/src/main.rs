use std::fs::File;
use std::io::{prelude::*, BufReader, self};
use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Clone, Debug)]
struct Map {
    board: Vec<Vec<char>>,
    height: usize,
    width: usize
}

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
    for i in 0..lines.len() {
        lines[i].push('.');
        lines[i].insert(0, '.');
    }
    lines.insert(0, vec!['.'; lines[1].len()]);
    lines.push(vec!['.'; lines[1].len()]);
    let (h, w) = (lines.len(), lines[1].len());
    let parsed = Map {
        board: lines,
        height: h,
        width: w,
    };
    parsed
}

/*
    I feel like breadth first is probably more efficient than depth first search,
    but I found this easier to write and it isn't that bad for part 1.
*/

fn dfs(g: Map, v: (i32, i32)) -> (Vec<(i32,i32)>, i32, Vec<char>) {
    let mut stack = vec![];
    let mut visited = vec![];
    let mut route = vec![];
    let mut chars = vec![];
    let mut count = 0;
    let check = HashMap::from([
        ((-1,0), vec!['|','F','7','S']),
        ((0,1), vec!['-','J','7','S']),
        ((1,0), vec!['|','J','L','S']),
        ((0,-1), vec!['-','L','F','S']),
        ]);
    stack.push(v);
    while let Some(current) = stack.pop() {
        stack.pop();
        if !visited.contains(&current) {
            visited.push(current);
            route.push(current);
            let current_char = g.board[current.0 as usize][current.1 as usize];
            chars.push(current_char);
            if current_char == 'S' && visited.len() > 1 {
                break;
            }
            let dirs: Vec<(i32, i32)> = match current_char {
                '|' => vec![(-1,0),(1,0)],
                '-' => vec![(0,-1),(0,1)],
                'F' => vec![(0,1),(1,0)],
                '7' => vec![(0,-1),(1,0)],
                'J' => vec![(0,-1),(-1,0)],
                'L' => vec![(-1,0),(0,1)],
                'S' => vec![(-1,0), (0,1), (1,0), (0,-1)],
                _ => panic!("Invalid char")
            };
            for (i,j) in dirs {
                let new = (current.0 as i32 + i, current.1 as i32 + j);
                let new_char = g.board[new.0 as usize][new.1 as usize];
                let vals = check.get(&(i,j)).unwrap().to_owned();
                if vec!['F','7','J','L'].contains(&current_char) &&
                   vec!['F','7','J','L'].contains(&new_char) && current_char == new_char {
                    continue;
                }
                if vals.contains(&new_char) && !visited.contains(&new) {
                    stack.push(new);
                    count += 1;
                }
            }
        }
    }
    (route, count, chars)
}

fn ray_cast(input: Vec<Vec<char>>) -> i32 {
    let mut board = input.clone();

    for l in 0..board.len() {
        for c in 0..board[0].len() {
            if board[l][c] != '.' {
                break;
            } else {
                board[l][c] = ' '
            }
        }
        for c in (0..board[0].len()).rev() {
            if board[l][c] != '.' {
                break;
            } else {
                board[l][c] = ' '
            }
        }
    }

    let mut part_2 = 0;

    for i in 1..board.len() {
        for j in (1..board[0].len()).rev() {
            if board[i][j] == '.' {
                let mut crossed = 0;
                let (mut x, mut y) = (j - 1, i - 1);

                while x > 0 && y > 0 {
                    if vec!['╯','╭','│','─','S'].contains(&board[y][x]) {
                        crossed += 1
                    }
                    x -= 1;
                    y -= 1;
                }

                if crossed % 2 == 0 {
                    board[i][j] = 'O';
                } else {
                    board[i][j] = '1';
                    part_2 += 1
                }
            }
        }
    }
    /*for b in board.clone() {
        let mut not_empty = false;
        for l in &b {
            if vec!['╮','╯','╭','╰','│','─'].contains(l) {
                not_empty = true;
            }
        }
        if not_empty == true {
            println!("{}", b.iter().collect::<String>())
        }
    }*/
    part_2
}

fn parts(data: Map) -> (i32, i32) {
    let mut results = (vec![(0,0)], 0, vec![' ']);
    for i in 0..data.height {
        for j in 0..data.width {
            if data.board[i][j] == 'S' {
                results =  dfs(data.clone(), (i as i32,j as i32))
            }
        }
    }

    let part_1 = results.1 / 2;

    let replace = HashMap::from([
        ('-', '─'),
        ('|', '│'),
        ('F', '╭'),
        ('7', '╮'),
        ('J', '╯'),
        ('L', '╰'),
    ]);

    for c in results.2.iter_mut() {
        if let Some(&replacement) = replace.get(&c) {
            *c = replacement;
        }
    }

    let mut new_board = vec![vec!['.'; data.width];data.height];
    for i in 0..results.0.len() {
        new_board[results.0[i].0 as usize][results.0[i].1 as usize] = results.2[i]
    }

    let part_2 = ray_cast(new_board);

    (part_1, part_2)
}

fn main() -> io::Result<()> {
    let data = parser("./input");
    let result = parts(data);
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);

    Ok(())
}
