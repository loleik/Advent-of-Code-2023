use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec;
use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Debug)]
struct Data {
    workflows: HashMap<String, Vec<String>>,
    values: Vec<Vec<i32>>
}

fn parser(path: &str) -> Data {
    let re_i = Regex::new(
        r"^[a-z]+\{((?:[a-z]+(<|>)\d+\:[a-zA-Z]+,)*[a-z]+(<|>)\d+\:[a-zA-Z]+)|((?:,+[a-zA-Z])*[a-zA-Z]+)\}$"
    ).unwrap();

    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };

    let reader = BufReader::new(file);

    let mut vals = vec![];

    let mut instr_map: HashMap<String, Vec<String>> = HashMap::new();

    reader.lines()
          .for_each(|line|
            if let Ok(l) = line {
                if re_i.is_match(&l) {
                    let x = l.split(['{', '}', ','])
                                          .filter(|&x| x.len() > 0)
                                          .map(|x| x.to_string())
                                          .collect::<Vec<_>>();
                    instr_map.insert(x[0].clone(), x[1..].to_vec());
                } else if l.len() != 0 {
                    vals.push(
                        l.split(['{', '}', ','])
                        .filter(|&x| x.len() > 0)
                        .map(|x| x.chars().skip(2).collect::<String>().parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                    );
                }
          });

    let data = Data { workflows: instr_map, values: vals };
    data
}

fn workflow(vals: &[i32], workflows: &HashMap<String, Vec<String>>) -> bool {
    let mut vals_map: HashMap<char, i32> = HashMap::new();
    let mut current = workflows.get("in").unwrap();
    let re_c = Regex::new(r"^([a-z]+(<|>)+\d+):([a-zA-Z]+)$").unwrap();
    let (mut a_r, mut term) = (false, false);

    for (i, v) in vals.iter().enumerate() {
        match i {
            0 => vals_map.insert('x', *v),
            1 => vals_map.insert('m', *v),
            2 => vals_map.insert('a', *v),
            3 => vals_map.insert('s', *v),
            _ => panic!("Invalid index")
        };
    }

    while term == false {
        for c in current {
            if let Some(captures) = re_c.captures(c) {
                let condition = captures.get(1).unwrap().as_str().chars().collect::<Vec<_>>();
                let next = captures.get(3).unwrap().as_str();
                let target = vals_map.get(&condition[0]).unwrap();
                
                match condition[1] {
                    '<' => {
                        if target < &condition[2..].iter().collect::<String>().parse::<i32>().unwrap() {
                            match next {
                                "A" => {
                                    a_r = true;
                                    term = true;
                                    break;
                                },
                                "R" => {
                                    term = true;
                                    break;
                                },
                                _ => {
                                    current = workflows.get(next).unwrap();
                                    break;
                                }
                            }
                        } else { continue; }
                    },
                    '>' => {
                        if target > &condition[2..].iter().collect::<String>().parse::<i32>().unwrap() {
                            match next {
                                "A" => {
                                    a_r = true;
                                    term = true;
                                    break;
                                },
                                "R" => {
                                    term = true;
                                    break;
                                },
                                _ => {
                                    current = workflows.get(next).unwrap();
                                    break;
                                }
                            }
                        } else { continue; }
                    },
                    _ => panic!("Invalid comparison operator: {}", condition[1])
                }
            } else {
                match c.as_str() {
                    "A" => {
                        term = true;
                        a_r = true;
                        break;
                    },
                    "R" => {
                        term = true;
                        break;
                    },
                    _ => {
                        current = workflows.get(c).unwrap();
                        break;
                    }
                }                
            }
        };
    }

    a_r
}

fn part_1(input: &Data) -> i32 {
    let mut results = vec![];
    let mut total = 0;

    input.values.iter().for_each(|i| {
        results.push(workflow(i, &input.workflows));
    });

    for (i, r) in results.iter().enumerate() {
        match r {
            true => total += input.values[i].iter().sum::<i32>(),
            _ => continue
        }
    }

    total
}

fn main() {
    let data = parser("./input");

    let part1 = part_1(&data);

    println!("Part 1: {part1}")
}
