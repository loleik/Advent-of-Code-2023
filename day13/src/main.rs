use std::fs::File;
use std::io::{prelude::*, BufReader, self};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Data {
    strings: Vec<Vec<char>>,
    seperators: Vec<usize>,
}

fn parser(path: &str) -> Data {
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
                .into_iter().map(|x| x.to_owned())
                .collect()
        })
        .collect();
    lines.insert(0, vec![]);
    lines.push(vec![]);
    
    let mut s = vec![];
    
    for l in 0..lines.len() {
        if lines[l].len() == 0 { s.push(l) }
    }
    
    let parsed = Data { strings: lines, seperators: s };
    parsed
}    

fn check_horizontal(input: Vec<Vec<char>>, mut check: Vec<usize>, dots: i32) -> Vec<usize> {
    if check.len() == 0 { return vec![] }

    let index = check.remove(0);

    let (mut up, mut down) = (index-1, index+1);
    let mut matches = 0;

    while down < input.len() {
        if input[up] != input[down] {
            matches = 0;
            break;
        }

        matches += 1;
        if up > 0 { up -= 1 } else { break; }
        down += 1;
    }

    let mut result = vec![];

    if matches != 0 {
        let dots_before_index = dots as usize - check.len() - 1;
        result.push(index - (dots_before_index) as usize)
    }

    let recursion = &check_horizontal(input.clone(), check, dots).to_owned();
    if recursion.len() > 0 { result.push(recursion[0]) }

    result
}

fn transpose(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if input.is_empty() || input[0].is_empty() {
        // Handle empty input gracefully
        return Vec::new();
    }

    let width = input[0].len();

    (0..width)
        .map(|col| input.iter().map(|row| row[col]).collect())
        .collect()
}

fn insert_dots(input: Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<usize>, i32) {
    let mut result = input.clone();
    let mut check = vec![];
    let mut dots = 0;

    for k in 0..(result.len()*2) {
        if k % 2 == 0 {
            result.insert(k, vec!['.'; result[0].len()]);
            if k != 0 {
                check.push(k-1);
                dots += 1
            }
        }
    }
    result.remove(0);
    (result, check, dots)
}

fn part_1(data: Data) -> (i64, HashMap<usize, i64>) {
    let mut results = HashMap::new();
    let s = data.seperators;
    let mut result = 0;
    let mut hori = false;
    let mut vert = false;

    for i in 0..(s.len()-1) {
        let current = data.strings[s[i]+1..s[i+1]].to_vec();
        let horizontal = insert_dots(current.clone());
        let current_hor = horizontal.0;
        let check_hor = horizontal.1;
        let dots_hor = horizontal.2;

        let horizontal_total = check_horizontal(current_hor.clone(), check_hor, dots_hor);

        let vertical = insert_dots(transpose(current));
        let current_ver = vertical.0;
        let check_ver = vertical.1;
        let dots_ver = vertical.2;

        let vertical_total = check_horizontal(current_ver, check_ver, dots_ver);

        let total = (100*horizontal_total.iter().sum::<usize>() + vertical_total.iter().sum::<usize>()) as i64;
        
        results.insert(i, total);
        
        result += total;
    }
    (result, results)
}

fn part_2(data: Data, results: HashMap<usize, i64>) {
    let s = data.seperators;
    let mut values = vec![];

    for i in 0..(s.len()-1) {
        let current = data.strings[s[i]+1..s[i+1]].to_vec();
        let mut passed = false;            
        
        for j in 0..current.len() {
            for k in 0..current[0].len() {
                let mut new_current = current.clone();

                match current[j][k] {
                    '.' => new_current[j][k] = '#',
                    '#' => new_current[j][k] = '.',
                    _ => panic!("Invalid character")
                }

                let horizontal = insert_dots(new_current.clone());
                let current_hor = horizontal.0;
                let check_hor = horizontal.1;
                let dots_hor = horizontal.2;

                let horizontal_total = check_horizontal(current_hor.clone(), check_hor, dots_hor);

                let vertical = insert_dots(transpose(new_current));
                let current_ver = vertical.0;
                let check_ver = vertical.1;
                let dots_ver = vertical.2;

                let vertical_total = check_horizontal(current_ver, check_ver, dots_ver);
                
                let r = (100*horizontal_total.iter().sum::<usize>() +
                         vertical_total.iter().sum::<usize>()) as i64;
                
                if results.get(&i).unwrap().to_owned() != r && r != 0 {
                    if results.get(&i).unwrap().to_owned() < r {
                        values.push(r - results.get(&i).unwrap().to_owned());
                        passed = true;
                        break;
                    } else {
                        values.push(results.get(&i).unwrap().to_owned() - r);
                        passed = true;
                        break;
                    }
                }
            }
            if passed == true { break; }
        }
        if passed == false {
            values.push(results.get(&i).unwrap().to_owned())
        }
    }
    println!("{:?},{}", values, values.iter().sum::<i64>())
}

fn main() {
    let data = parser("./input");
    let results = part_1(data.clone()).1;
    let result = part_1(data.clone()).0;
    println!("Part 1: {}", result);
    part_2(data, results)
}
