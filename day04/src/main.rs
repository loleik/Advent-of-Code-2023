use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Cards {
    list: Vec<Vec<String>>,
    number: usize,
}

fn parse(reader: BufReader<File>) -> Vec<Vec<String>> {
    let mut cards: Vec<Vec<String>> = vec![];
    for line in reader.lines() {
        let l = match line {
            Ok(line) => line,
            Err(e) => panic!("Error: {:?}", e),
        };
        cards.push(
            l.split(" ")
             .map(String::from)
             .collect::<Vec<_>>()
             .drain(2..)
             .collect()
        );
    }
    cards    
}

fn part_1(input: Vec<Vec<String>>) -> i32 {
    let mut total = 0;
    for mut c in input {
        c.retain(|s| s != "");
        if let Some(pipe_index) = c.iter().position(|x| x == "|") {
            let winning_numbers = &c[0..pipe_index];
            let your_numbers = &c[pipe_index + 1..];
            let matches = winning_numbers.iter()
                                                .filter(|x| your_numbers.contains(&x))
                                                .count();
            if matches > 0 {
                total += 2_i32.pow(u32::try_from(matches - 1).unwrap())
            }
        } else {
            println!("| not found in the vector");
        }
    }
    total
}

fn part_2(input: Vec<Vec<String>>, initial: usize) -> u32 {
    let list = input.clone();
    let copies: Vec<u32> = vec![1; initial];

    fn count_matches(mut card: Vec<Vec<String>>, copies: Vec<u32>, index: usize, max_index: usize) -> u32 {
        let mut modifier = copies;
        card[index].retain(|s| s != "");
        if let Some(pipe_index) = card[index].iter().position(|x| x == "|") {
            let winning_numbers = &card[index][0..pipe_index];
            let your_numbers = &card[index][pipe_index + 1..];
            let matches = winning_numbers.iter()
                                                .filter(|x| your_numbers.contains(&x))
                                                .count();
            for j in index+1..index+matches+1 {
                modifier[j] += 1 * modifier[index]
            }
        } else {
            println!("| not found in the vector");
        }
        if index < max_index {
            let new_list = card.clone();
            count_matches(new_list,modifier, index + 1, max_index)
        } else {
            let sum: u32 = modifier.iter().sum();
            sum
        }
    }
    count_matches(list, copies, 0, initial-1)
}

fn main() -> io::Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    let parsed = parse(reader);
    let length = parsed.len();
    let cards = Cards {
        list: parsed,
        number: length,
    };
    println!("Part 1: {} Part 2: {}", part_1(cards.list.clone()), part_2(cards.list, cards.number));
    Ok(())
}
