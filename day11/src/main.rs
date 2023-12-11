use std::fs::File;
use std::io::{prelude::*, BufReader, self};

#[derive(Clone, Debug)]
struct Map {
    map: Vec<Vec<char>>,
    horizontal: Vec<usize>,
    vertical: Vec<usize>,
    original_galaxies: Vec<(usize, usize)>,
    galaxies: Vec<(usize, usize)>,
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
    let mut ex_ver = vec![];
    let mut ex_hor = vec![];
    let mut galax = vec![];
    let mut galax_original = vec![];
    for i in 0..lines.len() {
        if !lines[i].contains(&'#') {
            ex_hor.push(i)
        }
    }
    let h = ex_hor.clone();
    for i in 0..lines[0].len() {
        let mut galaxy = false;
        for j in 0..lines.len() {
            if lines[j][i] == '#' {
                galax_original.push((j, i));
                galaxy = true
            }
        }
        if galaxy == false {
            ex_ver.push(i)
        }
    }
    let v = ex_ver.clone();
    for x in 0..ex_hor.len() {
        ex_hor[x] += x;
        lines.insert(ex_hor[x], vec!['X'; lines[0].len()]);
    }
    for x in 0..ex_ver.len() {
        ex_ver[x] += x;
        for l in 0..lines.len() {
            lines[l].insert(ex_ver[x], 'X');
        }
    }
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i][j] == '#' {
                galax.push((i, j))
            }
        }
    }
    galax.sort_by(|a, b| a.0.cmp(&b.0));
    let parsed = Map {
        map: lines,
        horizontal: h,
        vertical: v,
        original_galaxies: galax_original,
        galaxies: galax,
    };
    parsed
}

fn part_1(g: Vec<(usize, usize)>) -> i64 {
    let mut total = 0;
    for i in 0..g.len() {
        for j in i+1..g.len() {
            let pair = (g[i], g[j]);
            total += (pair.0.0 as i64 - pair.1.0 as i64).abs() + (pair.0.1 as i64 - pair.1.1 as i64).abs();
        }
    }
    total
}

// Could modify this to have variable expansion rate
fn part_2(g: Vec<(usize, usize)>, h: Vec<usize>, v: Vec<usize>) -> i64 {
    let mut g_expanded = g.clone();
    for i in 0..g.len() {
        for j in 0..h.len() {
            if g[i].0 > h[j] {
                g_expanded[i].0 += 999999
            }
        }
        for j in 0..v.len() {
            if g[i].1 > v[j] {
                g_expanded[i].1 += 999999
            }
        }
    }
    part_1(g_expanded)
}

fn main() {
    let data = parser("./input");
    println!("{}", part_1(data.galaxies));
    println!("{}", part_2(data.original_galaxies, data.horizontal, data.vertical))
}
