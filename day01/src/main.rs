use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    
    let mut z = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            let filtered: String = l.replace("one", "o1e")
                                    .replace("two", "t2o")
                                    .replace("three", "t3e")
                                    .replace("four", "f4r")
                                    .replace("five", "f5e")
                                    .replace("six", "s6x")
                                    .replace("seven", "s7n")
                                    .replace("eight", "e8t")
                                    .replace("nine", "n9e")
                                    .chars()
                                    .filter(|c| c.is_digit(10))
                                    .collect();
            let mut x = filtered.chars().nth(0).unwrap().to_string();
            let y = filtered.chars().nth(filtered.len()-1).unwrap();
            x.push(y);
            z += x.parse::<i32>().unwrap();
        } else {
            println!("Line error!");
            break;
        }
    }
    println!("{}", z);
    Ok(())
}
