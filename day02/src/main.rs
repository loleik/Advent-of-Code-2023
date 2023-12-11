use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_1(line: &String) -> bool {
    let mut sets = line.split(";").collect::<Vec<_>>();
    let s0 = sets[0].split(":").collect::<Vec<_>>();
    sets[0] = s0[1];
    let mut broken = false;
    for s in sets {
        let games = s.replace(" red", "r")
                                .replace(" green", "g")
                                .replace(" blue", "b")
                                .split(",")
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>();
        for x in games {
            if x.contains("r") {
                let r = &x[0..x.len()-1].to_string().trim().parse::<i32>().unwrap();
                if r > &12 { broken = true; }
            }
            if x.contains("g") {
                let g = &x[0..x.len()-1].to_string().trim().parse::<i32>().unwrap();
                if g > &13 { broken = true; }
            }
            if x.contains("b") {
                let b = &x[0..x.len()-1].to_string().trim().parse::<i32>().unwrap();
                if b > &14 { broken = true; }
            }
        }
    }
    broken
}

fn part_2(line: &String) -> i32 {
    let mut sets = line.split(";")
                                    .map(|x| x.replace(" ", ""))
                                    .map(|x| x.replace("red", "r"))
                                    .map(|x| x.replace("green", "g"))
                                    .map(|x| x.replace("blue", "b"))
                                    .collect::<Vec<_>>();
    let s0 = sets[0].split(":").collect::<Vec<_>>();
    sets[0] = s0[1].to_string();
    let mut full_split: Vec<&str> = Vec::new();
    for s in 0..sets.len() {
        full_split.append(&mut sets[s].split(",").collect());
    }
    let mut r_s: Vec<i32> = vec![];
    let mut g_s: Vec<i32> = vec![];
    let mut b_s: Vec<i32> = vec![];
    for f in full_split {
        if f.contains("r") { r_s.push(f.replace("r", "").parse::<i32>().unwrap()) }
        if f.contains("g") { g_s.push(f.replace("g", "").parse::<i32>().unwrap()) }
        if f.contains("b") { b_s.push(f.replace("b", "").parse::<i32>().unwrap()) }
    }
    let min_r = r_s.iter().max().unwrap();
    let min_g = g_s.iter().max().unwrap();
    let min_b = b_s.iter().max().unwrap();
    min_r * min_g * min_b
}

fn main() -> io::Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    let mut i = 0;
    let mut result_1 = 0;
    let mut result_2 = 0;
    
    for line in reader.lines() {
        if let Ok(l) = line {
            i += 1;
            if part_1(&l) == false {
                result_1 += i
            }
            result_2 += part_2(&l)
        }
    }
    println!("Part 1: {}, Part 2: {}", result_1, result_2);
    Ok(())
}
