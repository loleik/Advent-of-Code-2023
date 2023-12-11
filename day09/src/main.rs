use std::fs::File;
use std::io::{prelude::*, BufReader, self};

fn parser(path: &str) -> Vec<Vec<i64>> {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    let reader = BufReader::new(file);
    let lines: Vec<Vec<i64>> = reader
        .lines()
        .map(|line| {
            line.expect("Line error")
                .split_whitespace()
                .map(|s| s.parse::<i64>().expect("Value is not an integer"))
                .collect()
        })
        .collect();
    lines
}

fn differentiate(seq: Vec<i64>, mut seqs: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut seq_new = vec![];
    for i in 0..seq.len()-1 {
        seq_new.push(seq[i+1] - seq[i])
    }
    if seq_new.iter().all(|&x| x == 0) {
        seq_new.push(0);
        seqs.push(seq_new.clone());
        return seqs
    } else {
        seqs.push(seq_new.clone());
    }
    differentiate(seq_new, seqs)
}

fn part_1(data: Vec<Vec<i64>>) -> i64{
    let mut seqs = vec![];
    let mut result = 0 as i64;
    for d in data {
        seqs = differentiate(d.clone(), vec![d]);
        for s in (1..seqs.len()-1).rev() {
            let next = seqs[s-1].last().unwrap().clone();
            let curr = seqs[s].last().unwrap().clone();
            seqs[s-1].push(next + curr);
        }
        result += seqs[0].last().unwrap().to_owned();
    }
    result
}

fn part_2(data: Vec<Vec<i64>>) -> i64 {
    let mut seqs = vec![];
    let mut result = 0 as i64;
    for d in data {
        seqs = differentiate(d.clone(), vec![d]);
        for s in (1..seqs.len()-1).rev() {
            let next = seqs[s-1].first().unwrap().clone();
            let curr = seqs[s].first().unwrap().clone();
            seqs[s-1].insert(0, next - curr);
        }
        result += seqs[0].first().unwrap().to_owned();
    }
    result
}

fn main() -> io::Result<()> {
    let data = parser("./input");
    println!("Part 1: {}", part_1(data.clone()));
    println!("Part 2: {}", part_2(data));
    
    Ok(())
}
