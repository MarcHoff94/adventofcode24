use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let filepath = "C:\\users\\marcu\\projects\\adventofcode24\\src\\input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let input: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|x| x.chars().collect())
        .collect();
    let pattern = ['X', 'M', 'A', 'S'];
    let directions: [(i32, i32); 8] = [
        (0, 1),  
        (0, -1), 
        (1, 0),  
        (-1, 0), 
        (1, 1),  
        (1, -1), 
        (-1, 1), 
        (-1, -1) 
    ];

    let mut result: i32 = 0;

    for (i_line, line) in input.iter().enumerate() {
        for (i_col, &c) in line.iter().enumerate() {
            if c == pattern[0] {
                for dir in directions.iter() {
                    if matches_pattern(&input, &pattern, i_line as i32, i_col as i32, *dir) {
                        result += 1;
                    }
                }
            }
        }
    }

    println!("{}", result);
}


fn matches_pattern(
    input: &[Vec<char>],
    pattern: &[char],
    start_x: i32,
    start_y: i32,
    dir: (i32, i32),
) -> bool {
    for (i, &p) in pattern.iter().enumerate() {
        let x = start_x + dir.0 * i as i32;
        let y = start_y + dir.1 * i as i32;

        if x < 0 || y < 0 || x >= input.len() as i32 || y >= input[0].len() as i32 {
            return false;
        }

        if input[x as usize][y as usize] != p {
            return false;
        }
    }

    true
}
