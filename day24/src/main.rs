use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::RangeInclusive;
use regex::Regex;
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords { x: i64, y: i64, z: i64 }

fn parser(path: &str) -> Vec<Vec<Coords>> {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    let reader = BufReader::new(file);
    let re = Regex::new(r"(\d+,\s*\d+,\s*\d+) @\s*(-?\d+,\s*-?\d+,\s*-?\d+)$").unwrap();
    let mut out = vec![];

    reader.lines().for_each(|line| {
        if let Ok(l) = line {
            if let Some(caps) = re.captures(&l) {
                let vec_1 = caps.get(1)
                                .unwrap()
                                .as_str()
                                .replace(" ", "")
                                .split(',')
                                .map(|x| x.parse::<i64>().unwrap())
                                .collect::<Vec<_>>();
                let vec_2 = caps.get(2)
                                .unwrap()
                                .as_str()
                                .replace(" ", "")
                                .split(',')
                                .map(|x| x.parse::<i64>().unwrap())
                                .collect::<Vec<_>>();
                let co_1 = Coords { x: vec_1[0], y: vec_1[1], z: vec_1[2] };
                let co_2 = Coords { x: vec_2[0], y: vec_2[1], z: vec_2[2] };
                out.push(vec![co_1, co_2]);
            }
        }
    });

    out.sort_by(|a, b| b[0].z.cmp(&a[0].z));    
    out
}

fn exit_calc(input: &[Vec<Coords>]) -> (Vec<Vec<RangeInclusive<i64>>>,Vec<Vec<(RangeInclusive<f64>,i64)>>) {
    let (mut x_dist, mut y_dist) = (0,0);
    let mut exit_ranges = vec![];
    let mut exit_times = vec![];

    input.iter().for_each(|i| {
        // Distance from coords to the boundaries of the test area
        match (i[1].x, i[1].y) {
            (x,y) if x > 0 && y > 0 => {
                x_dist = 27 - i[0].x;
                y_dist = 27 - i[0].y;

                exit_ranges.push(vec![(i[0].x..=27), (i[0].y..=27)]);
            },
            (x,y) if x < 0 && y > 0 => {
                x_dist = i[0].x - 7;
                y_dist = 27 - i[0].y;

                exit_ranges.push(vec![(7..=i[0].x), (i[0].y..=27)]);
            },
            (x,y) if x > 0 && y < 0 => {
                x_dist = 27 - i[0].x;
                y_dist = i[0].y - 7;

                exit_ranges.push(vec![(i[0].x..=27), (7..=i[0].y)]);
            },
            (x,y) if x < 0 && y < 0 => {
                x_dist = i[0].x - 7;
                y_dist = i[0].y - 7;

                exit_ranges.push(vec![(7..=i[0].x), (7..=i[0].y)]);
            },
            (_,_) => panic!("Unknown case: {:?}", i[1])
        }

        // Turns out all of these are 0 < t < 1
        let x_exit: f64 = i[1].x.abs() as f64 / x_dist as f64;
        let y_exit: f64 = i[1].y.abs() as f64 / y_dist as f64;

        // Maybe one range would be better
        exit_times.push(vec![((0 as f64)..=x_exit, x_dist),((0 as f64)..=y_exit, y_dist)]);
    });
    (exit_ranges, exit_times)
}

fn intersection(
    dist: &[Vec<RangeInclusive<i64>>], 
    input: &[Vec<Coords>],
    time: &[Vec<(RangeInclusive<f64>,i64)>]
) -> i32 {
    let mut intersections = 0;

    for i in 0..dist.len() {
        let (x_min, y_min) = (dist[i][0].start(), dist[i][1].start());
        let (x_max, y_max) = (dist[i][0].end(), dist[i][1].end());

        for j in i+1..dist.len() {
            if max(x_min, dist[j][0].start()) <= min(x_max, dist[j][0].end()) && 
               max(y_min, dist[j][1].start()) <= min(y_max, dist[j][1].end()) {
                let x = max(x_min, dist[j][0].start());
                let y = max(y_min, dist[j][1].start());

                let tx1 = input[i][1].x.abs() as f64 / time[i][0].1 as f64;
                let ty1 = input[i][1].y.abs() as f64 / time[i][1].1 as f64;
                let tx2 = input[j][1].x.abs() as f64 / time[j][0].1 as f64;
                let ty2 = input[j][1].x.abs() as f64 / time[j][1].1 as f64;

                if tx1 <= *time[i][0].0.end() && ty1 <= *time[i][1].0.end() &&
                   tx2 <= *time[j][0].0.end() && ty2 <= *time[j][1].0.end() {
                    intersections += 1;
                }
            }
        }
    }
    println!("{intersections}");
    intersections
}

fn main() {
    let data = parser("./input");
    let ranges = exit_calc(&data);
    intersection(&ranges.0, &data, &ranges.1);
}
