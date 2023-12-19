use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::RangeInclusive;
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

fn workflow(vals: &[RangeInclusive<i32>], workflows: &HashMap<String, Vec<String>>) -> bool {
    let mut vals_map: HashMap<char, i32> = HashMap::new();
    let mut start = workflows.get("in").unwrap();
    let (mut a_r, mut term) = (false, false);
    let mut range = false;

    if vals.iter().all(|x| x.clone().collect::<Vec<_>>().len() == (1 as usize)) {
        for (i, v) in vals.iter().enumerate() {
            match i {
                0 => vals_map.insert('x', *v.start()),
                1 => vals_map.insert('m', *v.start()),
                2 => vals_map.insert('a', *v.start()),
                3 => vals_map.insert('s', *v.start()),
                _ => panic!("Invalid index")
            };
        }
        range = false;
    } else {
        range = true;
        vals_map.insert('x', vals[0].start().clone());
        vals_map.insert('m', vals[1].start().clone());
        vals_map.insert('a', vals[2].start().clone());
        vals_map.insert('s', vals[3].start().clone());
    }

    a_r = traverse(range, start, vals_map, workflows);

    fn traverse(
        r: bool,
        c: &Vec<String>, 
        vals_map: HashMap<char, i32>,
        workflows: &HashMap<String, Vec<String>>
    ) -> bool {
        let re_c = Regex::new(r"^([a-z]+(<|>)+\d+):([a-zA-Z]+)$").unwrap();
        let mut accepted = false;
        let mut current = c.clone();
        let mut term = false;

        while term == false {
            for c in &current {
                if let Some(captures) = re_c.captures(&c) {
                    let condition = captures.get(1).unwrap().as_str().chars().collect::<Vec<_>>();
                    let next = captures.get(3).unwrap().as_str();
                    let target = vals_map.get(&condition[0]).unwrap();
                    
                    match condition[1] {
                        '<' => {
                            if target < &condition[2..].iter().collect::<String>().parse::<i32>().unwrap() {
                                match next {
                                    "A" => {
                                        accepted = true;
                                        term = true;
                                        break;
                                    },
                                    "R" => {
                                        term = true;
                                        break;
                                    },
                                    _ => {
                                        current = workflows.get(next).unwrap().to_owned();
                                        break;
                                    }
                                }
                            } else { continue; }
                        },
                        '>' => {
                            if target > &condition[2..].iter().collect::<String>().parse::<i32>().unwrap() {
                                match next {
                                    "A" => {
                                        accepted = true;
                                        term = true;
                                        break;
                                    },
                                    "R" => {
                                        term = true;
                                        break;
                                    },
                                    _ => {
                                        current = workflows.get(next).unwrap().to_owned();
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
                            accepted = true;
                            break;
                        },
                        "R" => {
                            term = true;
                            break;
                        },
                        _ => {
                            current = workflows.get(c).unwrap().to_owned();
                            break;
                        }
                    }                
                }
            };
        }

        accepted
    }

    a_r
}

fn part_1(input: &Data) -> i32 {
    let mut results = vec![];
    let mut total = 0;

    input.values.iter().for_each(|i| {
        let mut ranges = vec![];

        for j in 0..=3 {
            ranges.push(i[j]..=i[j]);
        }
        
        results.push(workflow(&ranges, &input.workflows));
    });

    for (i, r) in results.iter().enumerate() {
        match r {
            true => total += input.values[i].iter().sum::<i32>(),
            _ => continue
        }
    }

    total
}


// Got to go back to this. Refactored part 1 code to work with ranges but too tired to continue.
fn part_2(input: &HashMap<String, Vec<String>>) {
    let initial = vec![(1..=4000), (1..=4000), (1..=4000), (1..=4000)];

    workflow(&initial, input);
}

fn main() {
    let data = parser("./input");

    println!("Part 1: {}", part_1(&data));

    part_2(&data.workflows);
}
