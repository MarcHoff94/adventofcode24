use std::{fs::{File}, io::{BufReader, BufRead}};

fn main() {
    let filepath = "C:\\users\\marcu\\projects\\adventofcode\\day1\\src\\input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut leftlist: Vec<u32> = Vec::new();
    let mut rightlist: Vec<u32> = Vec::new();
    reader.lines().for_each(|x| {
        let line = x.unwrap();
        let mut splitstr = line.split("   ");
        leftlist.push(splitstr.next().unwrap().parse().unwrap());
        rightlist.push(splitstr.next().unwrap().parse().unwrap());

    });
    leftlist.sort_unstable();
    rightlist.sort_unstable();
    let mut result: u32 = 0;
    for (i, &value) in leftlist.iter().enumerate() {
        result += value.abs_diff(rightlist[i]);
    }
    println!("{}", result);
    // println!("leftlen: {}, rightlen: {}", leftlist.len(), rightlist.len());

}