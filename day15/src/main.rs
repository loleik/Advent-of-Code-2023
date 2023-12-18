use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap, BTreeMap};

fn parser(path: &str) -> Vec<Vec<char>> {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {e}"),
    };

    let reader = BufReader::new(file);

    let mut instructions = vec![];
    
    for r in reader.lines() {
        let l = r.expect("Line error");
        let split: Vec<&str> = l.split(',').collect();
        let chars: Vec<Vec<char>> = split.iter().map(|x| x.chars().collect()).collect();
        chars.iter().for_each(|x| instructions.push(x.clone()));
    }
    
    instructions
}

fn hash(input: &[char]) -> u32 {
    let mut current_value = 0;
    let ascii_vals: Vec<u32> = input.iter().map(|&c| c as u32).collect();
    
    for x in ascii_vals {
        current_value = (current_value + x) * 17 % 256;
    }
    
    current_value
}

fn main() {
    let data = parser("./input");
    let mut part_1 = 0;
    let mut boxes: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    let mut lenses: BTreeMap<String, (u32, u32, u32)> = BTreeMap::new();
    
    for d in &data { part_1 += hash(&d) }
    println!("Part 1: {part_1}");
    
    for d in &data {
        let label: Vec<char> = d.iter()
                .copied()
                .filter(|&c| c.is_alphabetic())
                .collect();
        let insert: String = d.iter()
                .copied()
                .map(|c| if c == '=' || c == '-' { ' ' } else { c } )
                .collect();
        let correct_box = hash(&label);
        let check = label.into_iter().collect::<String>();
        let mut length = 0;
        
        if !boxes.contains_key(&correct_box) { boxes.insert(correct_box, vec![]); }
        
        if d.contains(&'-') && boxes.get(&correct_box).is_some() {
            if let Some(v) = boxes.get_mut(&correct_box) {
                v.retain(|s| !s.contains(&check));
            }
            lenses.remove(&check);
        } else if d.contains(&'=') {
            length = d.iter()
                .copied()
                .filter(|&c| c.is_numeric())
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            boxes.entry(correct_box).and_modify(|v| {
                    if let Some(i) = v.iter().position(|s| s.contains(&check)) {
                        v[i] = insert.clone();
                        lenses.insert(check, (correct_box+1, 0, length));
                    } else {
                        v.push(insert.clone());
                        lenses.insert(check, (correct_box+1, 0, length));
                    }
            });
        }
    }
    
    let mut total: u64 = 0;
    
    for (k, values) in lenses.clone() {
        let mut slot = 0;
        for v in boxes.values() {
            if let Some(i) = v.iter().position(|s| s.contains(&k)) {
                slot = (i + 1) as u32;
            }
        };

        total += (values.0 * slot * values.2) as u64;
    }
    
    println!("{total}")
}
