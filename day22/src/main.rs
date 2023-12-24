use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords { x: i32, y: i32, z: i32 }

impl Coords {
    fn diff(self, other: Coords) -> Option<& 'static str> {
        if self.x != other.x { Some("x") }
        else if self.y != other.y { Some("y") }
        else if self.z != other.z { Some("z") }
        else { None }
    }
}

fn parser(path: &str) -> Vec<Vec<Coords>> {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    let reader = BufReader::new(file);
    let re = Regex::new(r"(\d+,\d+,\d+)\~(\d+,\d+,\d+)$").unwrap();
    let mut out = vec![];

    reader.lines().for_each(|line| {
        if let Ok(l) = line {
            if let Some(caps) = re.captures(&l) {
                let vec_1 = caps.get(1)
                                .unwrap()
                                .as_str()
                                .split(',')
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect::<Vec<_>>();
                let vec_2 = caps.get(2)
                                .unwrap()
                                .as_str()
                                .split(',')
                                .map(|x| x.parse::<i32>().unwrap())
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

fn overlap(v: &[Coords], w: &[Coords]) -> bool {
    (max(v[0].x, w[0].x) <= min(v[1].x, w[1].x)) &&
    (min(v[0].y, w[0].y) <= max(v[1].y, w[1].y))
}

fn deep_copy(v: &[Vec<Coords>]) -> Vec<Vec<Coords>> {
    v.iter().map(|inner| inner.iter().cloned().collect()).collect()
}

fn part_1(input: &[Vec<Coords>]) {
    let mut copy = deep_copy(input);

    for (i, v) in input.iter().enumerate() {
        let mut floor = 1;
        for (j, w) in input.iter().enumerate().skip(i+1) {
            if overlap(v, w) == true {
                floor = max(w[0].z + 1, floor);
            }
        }
        //println!("Before Modification - copy[{}]: {:?}", i, copy[i]);
        //println!("{floor}");
        copy[i][1].z -= copy[i][0].z - floor;
        copy[i][0].z = floor;
        //println!("After Modification - copy[{}]: {:?}", i, copy[i]);
    }

    copy.sort_by(|a, b| a[0].z.cmp(&b[0].z));

    for c in &copy { println!("{:?}", c) }
}

fn main() {
    let data = parser("./input");
    part_1(&data)
}
