use std::fs::File;
use std::io::{prelude::*, BufReader, self};
use std::collections::HashMap;
use fancy_regex::Regex;

/*
    Enjoyed thinking about todays challenge, but not happy with the code that resulted from it.
    Hey it works though.
*/

fn parser(path: &str) -> Vec<Vec<String>> {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    let reader = BufReader::new(file);
    let lines: Vec<Vec<String>> = reader.lines()
           .map(|line|
               line.expect("Line error")
                   .split_whitespace()
                   .collect::<Vec<_>>()
                   .iter()
                   .map(|l|
                       l.to_string()
                   ).collect()
           )
           .collect();
    lines
}

fn order_hands(mut data: Vec<Vec<String>>, values: HashMap<char, i32>) -> Vec<Vec<String>> {
    let mut swapped = true;
    while swapped == true {
        swapped = false;
        if data.len() == 1 {
            break;
        }
        for i in 0..data.len() - 1 {
            let current: Vec<char> = data[i][0].chars().collect();
            let next: Vec<char> = data[i + 1][0].chars().collect();
            for j in 0..5 {
                if values.get(&current[j]) < values.get(&next[j]) {
                    data.swap(i, i+1);
                    swapped = true;
                    break;
                } else if values.get(&current[j]) == values.get(&next[j]) {
                    continue;
                } else {
                    break;
                }
            }
        }
    }
    data
}

fn handle_joker(hand: Vec<String>) -> String {
    let mut char_count = HashMap::new();
    for c in hand[0].chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    if char_count.get(&'J').unwrap() == &5 {
        return "five".to_owned()
    }
    let max_char = char_count
                    .iter()
                    .filter(|&(ch, _)| *ch != 'J')
                    .max_by_key(|&(_, count)| count)
                    .map(|(ch, _)| *ch)
                    .unwrap();
    let j_count = char_count.get(&'J').unwrap().to_owned();
    let max_count = char_count.get(&max_char).unwrap().to_owned();
    let new_count = j_count + max_count;
    char_count.insert(max_char, new_count);
    char_count.insert('J', 0);
    let mut hand_type = "";
    if char_count.values().any(|&count| count == 5) {
        hand_type = "five";
    } else if char_count.values().all(|&count| count == 1) {
        hand_type = "high";
    } else if char_count.values().any(|&c| c == 4) {
        hand_type = "four";
    } else if char_count.values().any(|&c| c == 3) &&
              char_count.values().any(|&c| c == 2) {
        hand_type = "full";
    } else if char_count.values().any(|&c| c == 3) {
        hand_type = "three";
    } else if char_count.values().any(|&c| c == 2) &&
              char_count.len() == 3 {
        hand_type = "two";
    } else if char_count.values().any(|&c| c == 2) {
        hand_type = "one";
    } else {
        hand_type = "none";
    }
    hand_type.to_owned()
}

fn sort_hands(data: Vec<Vec<String>>, vals: HashMap<char, i32>, flag: i32) -> i32 {
    let five_kind_re = Regex::new(r"^([TJQKA1-9])\1{4}$").unwrap();
    let mut five_kind: Vec<Vec<String>> = vec![];
    let mut four_kind: Vec<Vec<String>> = vec![];
    let mut full_house: Vec<Vec<String>> = vec![];
    let mut three_kind: Vec<Vec<String>> = vec![];
    let mut two_pair: Vec<Vec<String>> = vec![];
    let mut one_pair: Vec<Vec<String>> = vec![];
    let high_card_re = Regex::new(r"^(?!.*(.).*\1)[TJQKA1-9]+$").unwrap();
    let mut high_card: Vec<Vec<String>> = vec![];
    let mut unmatched: Vec<Vec<String>> = vec![];

    for d in data {
        if flag == 2 && d[0].chars().any(|c| c == 'J') {
            match handle_joker(d.clone()).as_str() {
                "five" => five_kind.push(d),
                "four" => four_kind.push(d),
                "full" => full_house.push(d),
                "three" => three_kind.push(d),
                "two" => two_pair.push(d),
                "one" => one_pair.push(d),
                "high" => high_card.push(d),
                _ => unmatched.push(d)
            };
        } else {
            let mut char_count = HashMap::new();
            for c in d[0].chars() {
                *char_count.entry(c).or_insert(0) += 1;
            }
            if five_kind_re.is_match(&d[0]).unwrap() {
                five_kind.push(d)
            } else if high_card_re.is_match(&d[0]).unwrap() {
                high_card.push(d)
            } else if char_count.values().any(|&c| c == 4) {
                four_kind.push(d)
            } else if char_count.values().any(|&c| c == 3) &&
                      char_count.values().any(|&c| c == 2) {
                full_house.push(d)
            } else if char_count.values().any(|&c| c == 3) {
                three_kind.push(d);
            } else if char_count.values().any(|&c| c == 2) &&
                      char_count.len() == 3 {
                two_pair.push(d)
            } else if char_count.values().any(|&c| c == 2) {
                one_pair.push(d)
            } else {
                unmatched.push(d)
            }
        }
    }
    let mut sorted = vec![
        five_kind, four_kind, full_house, three_kind,
        two_pair, one_pair, high_card,
    ];
    sorted.retain(|s| s.len() != 0);
    let mut results: Vec<Vec<String>> = vec![];
    for s in sorted {
        results.extend(order_hands(s, vals.clone()));
    }
    let mut prod = 0;
    for i in 0..results.len() {
        prod += results[i][1].parse::<i32>().unwrap() * (results.len() - i) as i32
    }
    prod
}

fn part_1(data: Vec<Vec<String>>) -> i32 {
    let values = HashMap::from([
        ('A',14i32),('K',13i32),('Q',12i32),('J',11i32),('T',10i32),('9',9i32),('8',8i32),
        ('7',7i32),('6',6i32),('5',5i32),('4',4i32),('3',3i32),('2',2i32),
    ]);
    sort_hands(data, values, 1)
}

fn part_2(data: Vec<Vec<String>>) -> i32 {
    let values = HashMap::from([
        ('A',14i32),('K',13i32),('Q',12i32),('T',10i32),('9',9i32),('8',8i32),('7',7i32),
        ('6',6i32),('5',5i32),('4',4i32),('3',3i32),('2',2i32),('J',1i32),
    ]);
    sort_hands(data, values, 2)
}

fn main() -> io::Result<()> {
    let data = parser("./input");
    println!("Part 1: {}", part_1(data.clone()));
    println!("Part 2: {}", part_2(data));
    
    Ok(())
}
