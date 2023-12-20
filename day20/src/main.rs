use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;

static RE: &str = r"([%&])?([a-z]+)\s*->\s*([a-z]+(?:,\s[a-z]+)*)$";

struct  Data {
    descriptors: HashMap<Vec<String>,Vec<String>>,
    flipflops: HashMap<String,usize>,
    conjuncts: HashMap<String,Vec<usize>>
}

fn parser(path: &str) -> Data {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };

    let reader = BufReader::new(file);

    let mut mod_map = HashMap::new();
    let mut con_map = HashMap::new();
    let mut flf_map = HashMap::new(); 

    reader.lines().for_each(|l| {
        if let Ok(line) = l {
            if let Some(captures) = Regex::new(RE).unwrap().captures(&line) {
                let i = captures.get(1).map(|m| m.as_str().to_owned()).unwrap_or_default();
                let j = captures.get(2).map(|m| m.as_str().to_owned()).unwrap_or_default();
                let k = captures.get(3).map(|m| m.as_str().to_owned()).unwrap_or_default();
                let k_vals: Vec<String> = k.split(',').map(|s| s.trim().to_owned()).collect();
                mod_map.insert(vec![i.clone(), j.clone()], k_vals);
                if &i == "%" { flf_map.insert(j.clone(), 0 as usize); }
                if &i == "&" { con_map.insert(j, vec![0 as usize,0 as usize]); }
            }
        }
    });

    let out = Data { descriptors: mod_map, flipflops: flf_map, conjuncts: con_map };

    out
}

// I've had to move on from this day. Even part 1 is just confusing me too much.
fn traverse(
    input: &Data, 
    v: &Vec<String>, 
    l: &Vec<String>, 
    c: i64, s: usize
) -> i64 {
    println!("{:?}:{:?}",l , v);
    let mut high = c;
    let mut flf = input.flipflops.clone();
    let mut con = input.conjuncts.clone();

    match l[0].as_str() {
        "%" => {
            let state = input.flipflops.get(&l[1]).unwrap();
            match (state, s) {
                (0,0) => {
                    flf.insert(l[1].clone(), 1);

                    let new = Data {
                        descriptors: input.descriptors.clone(),
                        flipflops: flf,
                        conjuncts: con
                    };
                    
                    for w in v {
                        if let Some((k,val)) = input.descriptors.iter().find(|(k, _)| k.get(1) == Some(&w.to_string())) {
                            high += traverse(&new, val, k, high, 1);
                        }
                    }
                },
                (1,0) => {
                    flf.insert(l[1].clone(), 0);

                    let new = Data {
                        descriptors: input.descriptors.clone(),
                        flipflops: flf,
                        conjuncts: con
                    };

                    for w in v {
                        if let Some((k,val)) = input.descriptors.iter().find(|(k, _)| k.get(1) == Some(&w.to_string())) {
                            high += traverse(&new, val, k, high, 0);
                        }
                    }
                },
                (1,1) | (0,1) => {
                    high += 1;
                },
                _ => panic!("Invalid combination")
            }
        },
        "&" => { println!("Conjunction {:?}", l) },
        "" => {
            for w in v {
                if let Some((k,val)) = input.descriptors.iter().find(|(k, _)| k.get(1) == Some(&w.to_string())) {
                    high += traverse(input, val, k, high, s);
                }
            }
        },
        _ => panic!("Invalid type indicator")
    }

    high
}

fn main() {
    let data = parser("./input");

    let start = data.descriptors.get(&vec!["".to_string(), "broadcaster".to_string()]).unwrap();

    traverse(&data, start, &vec!["".to_string(), "broadcaster".to_string()], 0, 0);
}
