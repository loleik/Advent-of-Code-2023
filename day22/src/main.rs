use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords { x: i32, y: i32, z: i32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Key {
    first: Coords,
    second: Coords,
}

impl Key {
    fn new(first: Coords, second: Coords) -> Self {
        Key { first, second }
    }
}

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

    out.sort_by(|a, b| a[0].z.cmp(&b[0].z));    
    out
}

// Same as 20 and 21, this has confused me too much. Maybe it could work but I can't continue.
fn part_1(input: &[Vec<Coords>]) {
    let mut free = 0;
    let mut supports: HashMap<Key, Vec<(Coords, Coords)>> = HashMap::new();
    let mut supporting: HashMap<Key, Vec<(Coords, Coords)>> = HashMap::new();

    for (i, v) in input.iter().enumerate() {
        let d = v[0].diff(v[1]).unwrap_or("Error finding differences");

        for (j, w) in input.iter().enumerate().skip(i+1) {
            match d {
                "x" => {
                    if ((v[0].x <= w[0].x && w[0].x <= v[1].x) ||
                       (v[0].x <= w[1].x && w[1].x <= v[1].x)) &&
                       ((w[0].y <= v[0].y && v[0].y <= w[1].y) ||
                       (w[0].y <= v[1].y && v[1].y <= w[1].y)) {
                        let entry1 = supports.entry(Key::new(w[0],w[1])).or_insert(vec![]);
                        entry1.push((v[0], v[1]));               
                        let entry2 = supporting.entry(Key::new(v[0],v[1])).or_insert(vec![]);     
                        entry2.push((w[0], w[1]));
                    } else { continue }
                },
                "y" => {
                    if ((v[0].y <= w[0].y && w[0].y <= v[1].y) ||
                       (v[0].y <= w[1].y && w[1].y <= v[1].y)) &&
                       ((w[0].x <= v[0].x && v[0].x <= w[1].x) ||
                       (w[0].x <= v[1].x && v[1].x <= w[1].x)) {
                        let entry = supports.entry(Key::new(w[0],w[1])).or_insert(vec![]);
                        entry.push((v[0], v[1]));                 
                        let entry2 = supporting.entry(Key::new(v[0],v[1])).or_insert(vec![]);     
                        entry2.push((w[0], w[1]));
                    } else { continue }
                },
                "z" => {
                    if v[0].x == w[0].x || v[0].x == w[1].x ||
                       v[1].x == w[0].x || v[1].x == w[1].x {
                        let entry = supports.entry(Key::new(w[0],w[1])).or_insert(vec![]);
                        entry.push((v[0], v[1]));                 
                        let entry2 = supporting.entry(Key::new(v[0],v[1])).or_insert(vec![]);     
                        entry2.push((w[0], w[1]));
                    } else if v[0].y == w[0].y || v[0].y == w[1].y ||
                              v[1].y == w[0].y || v[1].y == w[1].y {
                        let entry = supports.entry(Key::new(w[0],w[1])).or_insert(vec![]);
                        entry.push((v[0], v[1]));                 
                        let entry2 = supporting.entry(Key::new(v[0],v[1])).or_insert(vec![]);     
                        entry2.push((w[0], w[1]));
                    } else { continue }
                },
                _ => continue
            }
        }
    }

    for (key, val) in supporting.iter() {
        println!("{:?} : {:?}", key, val);
        if val.len() == 0 {
            free += 1;
        }
    }

    println!("{}", free)
}

fn main() {
    let data = parser("./input");
    part_1(&data)
}
