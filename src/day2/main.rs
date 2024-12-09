use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("C:\\Users\\marcu\\projects\\adventofcode\\day2\\src\\input").unwrap();
    let reader = BufReader::new(file);
    let mut safe_count: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut report: Vec<i32> = line.split_whitespace().map(|level| level.parse::<i32>().unwrap()).collect();
        let distances: Vec<i32> = calculate_distances(&report);
        if is_safe(&distances) || is_within_tolerance_safe(&mut report, distances) {
            safe_count += 1;
        }
        
    };
    println!("{}", safe_count);
        
}

fn calculate_distances(report: &Vec<i32>) -> Vec<i32> {
    report.windows(2).map(|w| w[1] - w[0]).collect()
}

fn is_safe(distances: &Vec<i32>) -> bool {
    let mut safe: bool = false;
    if distances.iter().all(|&x| x < 0 && x > -4) {
        safe = true;
    } else if distances.iter().all(|&x| x > 0 && x < 4) {
        safe = true;
    }
    return safe
}

//not proud of part2...
fn is_within_tolerance_safe(report: &mut Vec<i32>, distances: Vec<i32>) -> bool {
    let mut num_pos: i32 = 0;
    let mut num_neg: i32 = 0;
    let mut num_zero: i32 = 0;
    let mut error_index: usize = 0; 
    for (i, d) in distances.iter().enumerate() {
        if *d == 0 {
            num_zero += 1;
            error_index = i;
        } else if *d > 0 {
            num_pos += 1;
        } else if *d < 0 {
            num_neg += 1
        }
    }

    if num_zero > 1 {
        return false
    }else if num_pos > num_neg && num_neg + num_zero <= 1 {
        distances.iter().enumerate().for_each(|(i,d)| {
            if *d <= 0 || *d > 3 {
                error_index = i;
            }
        });
        return is_fixable_report(report, error_index)
    } else if num_pos < num_neg && num_pos + num_zero <= 1 {
        distances.iter().enumerate().for_each(|(i,d)| {
            if *d >= 0 || *d < -3{
                error_index = i;
            }
        });
        return is_fixable_report(report, error_index)
    }

    false

}

fn is_fixable_report(report: &mut Vec<i32>, error_index: usize) -> bool {
    let mut new_report = report.clone();
    new_report.remove(error_index);
    let mut distances = calculate_distances(&new_report);
    if is_safe(&distances) == false {
        report.remove(error_index+1);
        distances = calculate_distances(&report);
        return is_safe(&distances)
    }
    return true
}
