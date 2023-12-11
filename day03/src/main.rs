#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{prelude::*, BufReader, self};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
struct Board {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

lazy_static! {
    static ref LOOKUP: HashMap<String, (i32, i32)> = {
      let mut l = HashMap::new();
      l.insert("up".to_string(), (1i32, 0i32));
      l.insert("up_ri".to_string(), (1i32, 1i32));
      l.insert("ri".to_string(), (0i32, 1i32));
      l.insert("bo_ri".to_string(), (-1i32, 1i32));
      l.insert("bo".to_string(), (-1i32, 0i32));
      l.insert("bo_le".to_string(), (-1i32, -1i32));
      l.insert("le".to_string(), (0i32, -1i32));
      l.insert("up_le".to_string(), (1i32, -1i32));
      l
    };
}

fn parser(path: &str) -> Board {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    let reader = BufReader::new(file);
    let mut lines: Vec<Vec<char>> = reader.lines()
                                   .map(|line| line.expect("Line error")
                                                                        .chars()
                                                                        .collect())
                                   .collect();
    for l in &mut  lines {
        l.insert(0, '.');
        l.insert(0, '.');
        l.insert(0, '.');
        l.push('.');
        l.push('.');
        l.push('.')
    }
    lines.insert(0, vec!['.'; lines[0].len()]);
    lines.insert(0, vec!['.'; lines[0].len()]);
    lines.insert(0, vec!['.'; lines[0].len()]);
    lines.push(vec!['.'; lines[0].len()]);
    lines.push(vec!['.'; lines[0].len()]);
    lines.push(vec!['.'; lines[0].len()]);
    let board = Board {
        data: lines.clone(),
        height: lines.len(),
        width: lines[0].len()
    };
    board
}

fn part_1(board: Board) -> i32 {
    let re = Regex::new(r"\d").unwrap();
    let mut part_indexes = vec![];
    (1..board.height).for_each(|y| {
        (1..board.width).for_each(|x| {
            if re.is_match(&board.data[y][x].to_string()) {
                for (i, j) in LOOKUP.values() {
                    match (i.to_owned(), j.to_owned()) {
                        (-1, 0) => {
                            //if y > 0 {
                            if board.data[y-1][x] != '.' &&
                               !re.is_match(&board.data[y-1][x].to_string()) {
                                part_indexes.push((y, x))
                            }
                            //}
                        },
                        (-1, 1) => {
                            //if y > 0 && x < board.width - 1 {
                            if board.data[y-1][x+1] != '.' &&
                               !re.is_match(&board.data[y-1][x+1].to_string()) {
                                part_indexes.push((y, x))
                            }
                            //}
                        },
                        (0, 1) => {
                            //if x < board.width - 1 {
                            if board.data[y][x+1] != '.' &&
                               !re.is_match(&board.data[y][x+1].to_string()) {
                                part_indexes.push((y, x))
                            }
                            //}
                        },
                        (1, 1) => {
                            //if y < board.height - 1 && x < board.width - 1 {
                            if board.data[y+1][x+1] != '.' &&
                               !re.is_match(&board.data[y+1][x+1].to_string()) {
                                part_indexes.push((y, x))
                            }
                            //}
                        },
                        (1, 0) => {
                            //if y < board.height - 1 {
                            if board.data[y+1][x] != '.' &&
                               !re.is_match(&board.data[y+1][x].to_string()) {
                                part_indexes.push((y, x))
                            }
                            //}
                        },
                        (1, -1) => {
                            //if y < board.height - 1 && x > 0{
                            if board.data[y+1][x-1] != '.' &&
                               !re.is_match(&board.data[y+1][x-1].to_string()) {
                                part_indexes.push((y, x))
                            }
                            //}
                        },
                        (0, -1) => {
                            //if x > 0{
                            if board.data[y][x-1] != '.' &&
                               !re.is_match(&board.data[y][x-1].to_string()) {
                                part_indexes.push((y, x))
                            }
                            //}
                        },
                        (-1, -1) => {
                            //if y > 0 && x > 0{
                            if board.data[y-1][x-1] != '.' &&
                               !re.is_match(&board.data[y-1][x-1].to_string()) {
                                part_indexes.push((y, x))
                            }
                            //}
                        },
                        _ => println!("Not in LOOKUP {:?}", (i, j))
                    }
                }
            }
        })
    });
    let mut trimmed_indexes = vec![];
    (0..part_indexes.len()).for_each(|p| {
        if p == 0 {
            trimmed_indexes.push(part_indexes[0])
        } else if part_indexes[p].1 - 1 != part_indexes[p-1].1 &&
                  part_indexes[p] != part_indexes[p-1] {
            trimmed_indexes.push(part_indexes[p])
        }
    });
    let mut values = vec![];
    for val in trimmed_indexes {
        let mut current = board.data[val.0][val.1].to_string();
        if val.1 > 0 {
            if re.is_match(&board.data[val.0][val.1-1].to_string()) {
                current = board.data[val.0][val.1-1].to_string() + &current;
                if val.1 > 1 {
                    if re.is_match(&board.data[val.0][val.1-2].to_string()) &&
                       !re.is_match(&board.data[val.0][val.1+1].to_string()) {
                        current = board.data[val.0][val.1-2].to_string() + &current
                    }
                }
            }
        }
        if val.1 < board.width - 1 {
            if re.is_match(&board.data[val.0][val.1+1].to_string()) {
                current = current + &board.data[val.0][val.1+1].to_string();
                if val.1 < board.width - 2 {
                    let cond_1 = re.is_match(&board.data[val.0][val.1+2].to_string());
                    let cond_2 = re.is_match(&board.data[val.0][val.1+1].to_string());
                    if cond_1 && cond_2 {
                        current = current + &board.data[val.0][val.1+2].to_string();
                    }
                }
            }
        }
        values.push(current)
    }
    let mut total = 0;
    for v in values {
        total += v.parse::<i32>().unwrap();
    }
    total
}

// Not using any part 1 code because it's a hellscape
fn part_2(board: Board) -> i32 {
    let vals = vec!['0','1','2','3','4','5','6','7','8','9',];
    let mut result = 0;

    for i in 1..(board.data.len()-1) {
        for j in 1..(board.data[0].len()-1) {
            if board.data[i][j] == '*' {
                let mut numbers = vec![];
                let mut top = vec![];
                let mut bot = vec![];

                for k in vec![-1, 0, 1] {
                    top.push(board.data[i-1][(j as i32 + k as i32) as usize]);
                    bot.push(board.data[i+1][(j as i32 + k as i32) as usize])
                }

                if vals.contains(&top[1]) {
                    let mut curr = top[1].to_string();
                    if vals.contains(&top[0]) {
                        curr = top[0].to_string() + &curr;
                        if vals.contains(&board.data[i-1][j-2]) {
                            curr = board.data[i-1][j-2].to_string() + &curr
                        } else if vals.contains(&top[2]) {
                            curr += &top[2].to_string();
                        }
                    } else if vals.contains(&top[2]) {
                        curr += &top[2].to_string();
                        if vals.contains(&board.data[i-1][j+2]) {
                            curr += &board.data[i-1][j+2].to_string()
                        } else if vals.contains(&top[0]) {
                            curr = top[2].to_string() + &curr;
                        }
                    }
                    numbers.push(curr.parse::<i32>().expect("Not an integer"))
                } else if vals.contains(&top[0]) {
                    let mut curr = top[0].to_string();
                    if vals.contains(&board.data[i-1][j-2]) {
                        curr = board.data[i-1][j-2].to_string() + &curr;
                        if vals.contains(&board.data[i-1][j-3]) {
                            curr = board.data[i-1][j-3].to_string() + &curr
                        }
                    }
                    if vals.contains(&top[2]) {
                        let mut curr = top[2].to_string();
                        if vals.contains(&board.data[i-1][j+2]) {
                            curr += &board.data[i-1][j+2].to_string();
                            if vals.contains(&board.data[i-1][j+3]) {
                                curr += &board.data[i-1][j+3].to_string();
                            }
                        }
                        numbers.push(curr.parse::<i32>().expect("Not an integer"))
                    }
                    numbers.push(curr.parse::<i32>().expect("Not an integer"))
                } else if vals.contains(&top[2]) {
                    let mut curr = top[2].to_string();
                    if vals.contains(&board.data[i-1][j+2]) {
                        curr += &board.data[i-1][j+2].to_string();
                        if vals.contains(&board.data[i-1][j+3]) {
                            curr += &board.data[i-1][j+3].to_string();
                        }
                    }
                    numbers.push(curr.parse::<i32>().expect("Not an integer"))
                }

                if vals.contains(&bot[1]) {
                    let mut curr = bot[1].to_string();
                    if vals.contains(&bot[0]) {
                        curr = bot[0].to_string() + &curr;
                        if vals.contains(&board.data[i+1][j-2]) {
                            curr = board.data[i+1][j-2].to_string() + &curr
                        } else if vals.contains(&bot[2]) {
                            curr += &bot[2].to_string();
                        }
                    } else if vals.contains(&bot[2]) {
                        curr += &bot[2].to_string();
                        if vals.contains(&board.data[i+1][j+2]) {
                            curr += &board.data[i+1][j+2].to_string()
                        } else if vals.contains(&bot[0]) {
                            curr = bot[2].to_string() + &curr;
                        }
                    }
                    numbers.push(curr.parse::<i32>().expect("Not an integer"))
                } else if vals.contains(&bot[0]) {
                    let mut curr = bot[0].to_string();
                    if vals.contains(&board.data[i+1][j-2]) {
                        curr = board.data[i+1][j-2].to_string() + &curr;
                        if vals.contains(&board.data[i+1][j-3]) {
                            curr = board.data[i+1][j-3].to_string() + &curr
                        }
                    }
                    if vals.contains(&bot[2]) {
                         let mut curr = bot[2].to_string();
                         if vals.contains(&board.data[i+1][j+2]) {
                             curr += &board.data[i+1][j+2].to_string();
                             if vals.contains(&board.data[i+1][j+3]) {
                                 curr += &board.data[i+1][j+3].to_string();
                             }
                         }
                         numbers.push(curr.parse::<i32>().expect("Not an integer"))
                     }
                    numbers.push(curr.parse::<i32>().expect("Not an integer"))
                } else if vals.contains(&bot[2]) {
                    let mut curr = bot[2].to_string();
                    if vals.contains(&board.data[i+1][j+2]) {
                        curr += &board.data[i+1][j+2].to_string();
                        if vals.contains(&board.data[i+1][j+3]) {
                            curr += &board.data[i+1][j+3].to_string();
                        }
                    }
                    numbers.push(curr.parse::<i32>().expect("Not an integer"))
                }

                if vals.contains(&board.data[i][j+1]) {
                    let x = board.data[i][j+1];
                    if vals.contains(&board.data[i][j+2]) {
                        let y = x.to_string() + &board.data[i][j+2].to_string();
                        if vals.contains(&board.data[i][j+3]) {
                            let z = y + &board.data[i][j+3].to_string();
                            numbers.push(z.parse::<i32>().expect("Not and integer"))
                        } else {
                            numbers.push(y.parse::<i32>().expect("Not and integer"))
                        }
                    } else {
                        numbers.push(x.to_string().parse::<i32>().expect("Not and integer"))
                    }
                }

                if vals.contains(&board.data[i][j-1]) {
                    let x = board.data[i][j-1];
                    if vals.contains(&board.data[i][j-2]) {
                        let y = board.data[i][j-2].to_string() + &x.to_string();
                        if vals.contains(&board.data[i][j-3]) {
                            let z = board.data[i][j-3].to_string() + &y;
                            numbers.push(z.parse::<i32>().expect("Not and integer"))
                        } else {
                            numbers.push(y.parse::<i32>().expect("Not and integer"))
                        }
                    } else {
                        numbers.push(x.to_string().parse::<i32>().expect("Not and integer"))
                    }
                }
                if numbers.len() == 2 {
                    result += numbers[0] * numbers[1]
                }
            }
        }
    }
    result
}

fn main() -> io::Result<()> {
    let board = parser("./input");
    println!("Part 1: {}", part_1(board.clone()));
    println!("Part 2: {}", part_2(board));
    Ok(())
}
