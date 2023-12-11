use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug, Default, Clone)]
struct Almanac {
    seeds: Vec<String>, seed_soil: Vec<String>, soil_fert: Vec<String>, fert_water: Vec<String>,
    water_light: Vec<String>, light_temp: Vec<String>, temp_humid: Vec<String>, humid_loc: Vec<String>,
}

/*
    Should've used a hashmap from the start instead of a struct, but realised after
    writing all the parsing code which took longer than it should. Maybe later...
*/
impl Almanac {
    fn to_hashmap(&self) -> HashMap<&str, Vec<String>> {
        let mut map = HashMap::new();
        map.insert("seeds", self.seeds.clone());
        map.insert("seed_soil", self.seed_soil.clone());
        map.insert("soil_fert", self.soil_fert.clone());
        map.insert("fert_water", self.fert_water.clone());
        map.insert("water_light", self.water_light.clone());
        map.insert("light_temp", self.light_temp.clone());
        map.insert("temp_humid", self.temp_humid.clone());
        map.insert("humid_loc", self.humid_loc.clone());
        map
    }
}

fn parser(path: &str) -> Almanac {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    let reader = io::BufReader::new(file);
    let mut sections = vec![];
    let mut current: Vec<String> = vec![];
    for line in reader.lines() {
        let l = match line {
            Ok(val) => val,
            Err(e) => panic!("Error: {}", e),
        };
        if l.trim().is_empty() {
            if !current.is_empty() {
                sections.push(std::mem::take(&mut current));
            }
        } else {
            current.push(l)
        }
    }
    if !current.is_empty() {
        sections.push(current)
    }
    let mut almanac: Almanac = Default::default();

    for (i, v) in sections.into_iter().enumerate() {
        match i {
            0 => almanac.seeds = v,
            1 => almanac.seed_soil = v,
            2 => almanac.soil_fert = v,
            3 => almanac.fert_water = v,
            4 => almanac.water_light = v,
            5 => almanac.light_temp = v,
            6 => almanac.temp_humid = v,
            7 => almanac.humid_loc = v,
            _ => panic!("Index error: {}", i)
        }
    }
    almanac
}

fn part_1(almanac: Almanac) -> i64 {
    let almap = almanac.to_hashmap();
    let mut seed_str: Vec<_> = almap.get("seeds")
                                   .unwrap()
                                   .iter()
                                   .flat_map(|x| x.split_whitespace())
                                   .collect::<Vec<_>>();
    seed_str.remove(0);
    let seed_int: Vec<i64> = seed_str.iter().map(|s| s.to_string().parse::<i64>().unwrap()).collect();
    let soil = conversion(almap.clone(), seed_int, "seed_soil");
    let fertilizer = conversion(almap.clone(), soil, "soil_fert");
    let water = conversion(almap.clone(), fertilizer, "fert_water");
    let light = conversion(almap.clone(), water, "water_light");
    let temperature = conversion(almap.clone(), light, "light_temp");
    let humidity = conversion(almap.clone(), temperature, "temp_humid");
    let location = conversion(almap, humidity, "humid_loc");
    location.iter().min().unwrap().to_owned()
}

fn part_2(almanac: Almanac) -> i64 {
    let almap = almanac.to_hashmap();
    let mut seed_str: Vec<_> = almap.get("seeds")
                                   .unwrap()
                                   .iter()
                                   .flat_map(|x| x.split_whitespace())
                                   .collect::<Vec<_>>();
    seed_str.remove(0);
    let seed_int: Vec<i64> = seed_str.iter().map(|s| s.to_string().parse::<i64>().unwrap()).collect();
    let mut ranges = vec![];
    for i in (0..seed_int.len()).step_by(2) {
        ranges.push(seed_int[i]..=(seed_int[i] + seed_int[i+1] - 1));
    }

    let mut locations = vec![];

    (0..ranges.len()).for_each(|r| {
        let soil = conversion_2(
            almap.clone(), vec![ranges[r].clone()], "seed_soil"
        );
        let fertilizer = conversion_2(
            almap.clone(), soil, "soil_fert"
        );
        let water = conversion_2(
            almap.clone(), fertilizer, "fert_water"
        );
        let light = conversion_2(
            almap.clone(), water, "water_light"
        );
        let temperature = conversion_2(
            almap.clone(), light, "light_temp"
        );
        let humidity = conversion_2(
            almap.clone(), temperature, "temp_humid"
        );
        locations.push(conversion_2(
            almap.clone(), humidity, "humid_loc"
        ))
    });
    locations.iter()
     .flat_map(
         |ranges| ranges.iter().flat_map(
             |range| vec![range.start(), range.end()]
         )
     )
     .min().unwrap().to_owned()
}

fn split_range(set: Vec<i64>, range: RangeInclusive<i64>,) -> Vec<RangeInclusive<i64>>{
    let source_range = set[1]..=(set[1] + set[2]);
    if range.start() <= source_range.end() && source_range.start() <= range.end() {
        let start = std::cmp::max(range.start(), source_range.start());
        let end = std::cmp::min(range.end(), source_range.end());
        if start > range.start() && end < range.end() {
            let ranges = vec![
                start.to_owned()..=end.to_owned(),
            ];
            ranges
        } else if start > range.start() && end == range.end() {
            let ranges = vec![
                start.to_owned()..=range.end().to_owned(),
            ];
            ranges
        } else if start == range.start() && end < range.end() {
            let ranges = vec![
                range.start().to_owned()..=end.to_owned(),
            ];
            ranges
        } else {
            vec![range]
        }
    } else {
        vec![]
    }
}

fn conversion_2(almap: HashMap<&str, Vec<String>>, ranges_in: Vec<RangeInclusive<i64>>, step: &str) -> Vec<RangeInclusive<i64>> {
    let mut current_step = almap.get(step).cloned().unwrap();
    current_step.remove(0);
    let split_step: Vec<Vec<i64>> = current_step.iter()
    .map(
        |x| x.split_whitespace()
                      .collect::<Vec<_>>()
                      .iter()
                      .map(|x| x.to_string().parse::<i64>().unwrap())
                      .collect()
    )
    .collect();
    let mut converted = vec![];
    for range in ranges_in {
        for set in split_step.clone() {
            let ranges = split_range(set.clone(), range.clone());
            if ranges.len() == 0 { continue; }
            if ranges[0].start().to_owned() > set[1] && ranges[0].end().to_owned() < set[1] + set[2] {
                converted.push(
                    (set[0] + (ranges[0].start() - set[1]))..=(set[0] + (ranges[0].end() - set[1]))
                )
            } else if ranges[0].start().to_owned() > set[1] && set[1] + set[2] == ranges[0].end().to_owned() {
                converted.push(
                    (set[0] + (ranges[0].start() - set[1]))..=(set[0] + set[2])
                )
            } else if set[1] == ranges[0].start().to_owned() && ranges[0].end().to_owned() < set[1] + set[2] {
                converted.push(
                    (set[0])..=(set[0] + (ranges[0].end().to_owned() - set[1]))
                )
            } else {
                converted.push(
                    (set[0])..=(set[0] + set[2])
                )
            }
        }
    }
    converted
}

fn conversion(almap: HashMap<&str, Vec<String>>, vals: Vec<i64>, step: &str) -> Vec<i64> {
    let mut current_step = almap.get(step).cloned().unwrap();
    current_step.remove(0);
    let split_step: Vec<Vec<i64>> = current_step.iter()
    .map(
        |x| x.split_whitespace()
                      .collect::<Vec<_>>()
                      .iter()
                      .map(|x| x.to_string().parse::<i64>().unwrap())
                      .collect()
    )
    .collect();
    let mut converted: Vec<i64> = vec![];
    (0..vals.len()).for_each(|i| {
        let mut pushed = false;
        for set in split_step.clone() {
            if set[1] <= vals[i] && vals[i] <= set[1] + set[2] {
                converted.push(set[0] + (vals[i] - set[1]));
                pushed = true;
                break;
            }
        }
        if pushed == false {
            converted.push(vals[i])
        }
    });
    converted
}

fn main() -> io::Result<()> {
    let almanac = parser("./input");
    println!("Part 1: {}", part_1(almanac.clone()));
    println!("Part 2: {:?}", part_2(almanac));

    Ok(())
}
