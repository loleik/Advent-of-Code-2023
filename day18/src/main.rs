use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Debug)]
struct Plan {
    directions: Vec<String>,
    distances: Vec<i64>,
}

fn parser(path: &str) -> (Plan, Plan) {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };

    let reader = BufReader::new(file);

    let lines: Vec<Vec<String>> = reader
        .lines()
        .map(|line| {
            line.expect("Line error")
                .split_whitespace()
                .into_iter().map(|x| x.to_owned())
                .collect()
        })
        .collect();
    
    let (mut dirs, mut dists) = (vec![], vec![]);
    
    let (mut corrected_dirs, mut corrected_dists, mut replaced) =
            (vec![], vec![], vec![]);
    
    for i in 0..lines.len() {
        dirs.push(lines[i][0].clone());
        
        dists.push(lines[i][1].parse::<i64>().unwrap_or_else(|_|
            { panic!("Invalid distance: {} in {:?}", lines[i][1], lines[i]); }));
        
        let dist_c = &lines[i][2].chars().filter(|c| c.is_alphanumeric())
                           .collect::<Vec<_>>().iter().take(5).collect::<String>();
        
        let dir_c = &lines[i][2].chars().filter(|c| c.is_alphanumeric())
                           .collect::<Vec<_>>().iter().rev().take(1).collect::<String>();
        
        match i64::from_str_radix(dist_c, 16) {
            Ok(rgb_val) => { corrected_dists.push(rgb_val) },
            Err(e) => { panic!("Error: {e}") }
        }
        
        corrected_dirs.push(dir_c.parse::<i64>().unwrap_or_else(|_|
            { panic!("Invalid distance: {}", dir_c); }));
    }
    
    for x in 0..corrected_dirs.len() {
        match corrected_dirs[x] {
            0 => { replaced.push("R".to_string()) },
            1 => { replaced.push("D".to_string()) },
            2 => { replaced.push("L".to_string()) },
            3 => { replaced.push("U".to_string()) },
            _ => panic!("Invalid direction"),
        }
    }
    
    let plan = Plan {
        directions: dirs,
        distances: dists,
    };
    
    let corrected_plan = Plan {
        directions: replaced,
        distances: corrected_dists,
    };
    
    (plan, corrected_plan)
}

fn vertices(input: Plan) -> (Vec<(i64, i64)>, i64) {
    let mut current: (i64, i64) = (0,0);
    let mut v = vec![];
    let dirs = input.directions.clone();
    let dists = input.distances.clone();
    let mut border_dist = 0;
    
    for i in 0..dirs.len() {
        match dirs[i].as_str() {
            "R" => { current = (current.0 + dists[i], current.1) },
            "L" => { current = (current.0 - dists[i], current.1) },
            "U" => { current = (current.0, current.1 + dists[i]) },
            "D" => { current = (current.0, current.1 - dists[i]) },
            _ => panic!("Invalid direction"),
        }
        
        border_dist += dists[i];
        
        if i < dirs.len() - 1 {
            if dirs[i] != dirs[i+1] { v.push(current) }
        } else {
            if dirs[i] != dirs[0] {v.push(current)}
        }
    }
    
    (v, border_dist)
}

fn shoelace_picks(v: Vec<(i64, i64)>, t: i64) -> i64 {
    // First use the Shoelace theorem 
    let mut total = 0;
    
    for i in 0..v.len() {
        if i == v.len() - 1 {
            let (x_n, y_n) = (v[i].0, v[i].1);
            let (x_1, y_1) = (v[0].0, v[0].1);
            let det = (x_n * y_1) - (x_1 * y_n);
            total += det;
        } else {
            let (x_1, y_1) = (v[i].0, v[i].1);
            let (x_2, y_2) = (v[i+1].0, v[i+1].1);
            let det = (x_1 * y_2) - (x_2 * y_1);
            total += det;
        }
    }
    
    total = (total / 2).abs();
    
    // Now use Pick's theorem
    let area = total + (t / 2) + 1;
    
    area
}

fn main() {
    let data = parser("./input");
    
    let v_1 = vertices(data.0);
    let v_2 = vertices(data.1);
    
    let part_1 = shoelace_picks(v_1.0, v_1.1);
    let part_2 = shoelace_picks(v_2.0, v_2.1);
    
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
