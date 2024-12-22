use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

fn main() {
    let filepath = "C:\\users\\marcu\\projects\\adventofcode24\\src\\input.txt";
    let file = File::open(filepath).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut input: Vec<Vec<i32>> = Vec::new();
    let mut result: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.contains('|') {
            let mut numstr = line.split('|');
            let key = numstr.next().unwrap().trim().parse::<i32>().unwrap();
            let value = numstr.next().unwrap().trim().parse::<i32>().unwrap();

            rules.entry(key).or_insert_with(Vec::new).push(value);
        } else if !line.trim().is_empty() {
            let update = line
                .split(',')
                .map(|x| x.trim().parse::<i32>().expect("Invalid number"))
                .collect::<Vec<_>>();
            input.push(update);
        }
    }

    for mut u in input {
        if check_update(&u, &rules) == false {
            fix_update(&mut u, &rules);
            let idx = u.len() / 2;
            result.push(u[idx]);
        }
    }

    let sum: i32 = result.iter().sum();
    println!("result: {}", sum);
}

fn check_update(update: &[i32], ruleset: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, &number) in update.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if let Some(rule_values) = ruleset.get(&number) {
            if rule_values.iter().any(|&x| update[..i].contains(&x)) {
                return false;
            }
        }
    }
    true
}

fn fix_update(update: &mut Vec<i32>, ruleset: &HashMap<i32, Vec<i32>>) {
    let mut i = 1; 
    while i < update.len() {
        let number = update[i];
        if let Some(rule_values) = ruleset.get(&number) {
            for &value in rule_values {
                if let Some(index) = update[..i].iter().position(|&x| x == value) {
                    let illegal_value = update.remove(index);
                    if i + 1 > update.len() {
                        update.insert(i, illegal_value);
                    } else {
                        update.insert(i + 1, illegal_value); 
                    }
                    i = 1;
                    break;
                }
            }
        }
        i += 1; 
    }
}
