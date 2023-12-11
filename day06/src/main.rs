use std::fs::File;
use std::io::{prelude::*, BufReader, self};

/*
    Equations:  D = hold(T - hold) = -hold^2 + T*hold
         dD/dhold = T - 2*hold
    For maximum: hold = T / 2
*/

macro_rules! quadratic_formula {
    ($a:expr, $b:expr, $disc:expr) => {
        {
            let result_1 = ( -$b + $disc.sqrt() ) / ( 2 as f64 * $a );
            let result_2 = ( -$b - $disc.sqrt() ) / ( 2 as f64 * $a );
            (result_1, result_2)
        }
    };
}

#[derive(Debug, Clone)]
struct Coefficients {
    a: i64, // -1
    b: i64, // T
    disc: i64, // b^2 - 4ac
}

#[derive(Debug, Clone)]
struct Data {
    times: Vec<i64>,
    records: Vec<i64>,
}

fn parser(path: &str) -> Data {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
                                   .map(|line| line.expect("Line error"))
                                   .collect();
    let t: Vec<i64> = lines[0].split_whitespace()
                                  .skip(1)  // Skip the first element
                                  .map(|t| t.parse::<i64>())
                                  .filter_map(Result::ok)
                                  .collect();
    let r: Vec<i64> = lines[1].split_whitespace()
                                    .skip(1)  // Skip the first element
                                    .map(|t| t.parse::<i64>())
                                    .filter_map(Result::ok)
                                    .collect();
    let data = Data {
        times: t,
        records: r,
    };
    data
}

fn part_1(input: Data) -> i64 {
    let possibilities: Vec<i64> = vec![];
    fn calculate(input: Data, index: usize, mut result: Vec<i64>) -> i64 {
        let (current_time, current_record) = (input.times[index], input.records[index]);
        
        let discriminant = current_time.pow(2) - 4 * (-1) * (-current_record);
        let coefs  = Coefficients { a: -1, b: current_time, disc: discriminant };
        
        // This will give the lowest and highest hold time to beat the record
        if discriminant >= 0 {
            let (root1, root2) = quadratic_formula!(
                coefs.a as f64, coefs.b as f64, coefs.disc as f64
            );
            result.push(root2.ceil() as i64 - root1.floor() as i64 - 1)
        } else { println!("Discriminant {} < 0", discriminant) }
        
        if index < input.times.len() - 1 {
            calculate(input, index + 1, result)
        } else {
            result.iter().fold(1, |acc, x| acc * x)
        }
    }
    calculate(input, 0, possibilities)
}

fn part_2(input: Data) -> i64 {
    let (times, records) = (input.times, input.records);
    let concat_times = vec![
        times.iter().fold(0, |acc, &x| acc * 100 + x)
    ];
    let concat_records = vec![
        records.iter().fold(0, |acc, &x| acc * 10000 + x)
    ];
    let concat_data = Data {
        times: concat_times, records: concat_records
    };
    part_1(concat_data)
}

fn main() -> io::Result<()> {
    let data = parser("./input");
    println!("Part 1: {}", part_1(data.clone()));
    println!("Part 2: {}", part_2(data));
    
    Ok(())
}
