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

    let pattern_straight = ['X', 'M', 'A', 'S'];
    let directions_straight: [(i32, i32); 8] = [
        (0, 1),  (0, -1), (1, 0), (-1, 0),
        (1, 1),  (1, -1), (-1, 1), (-1, -1)
    ];

    let pattern_cross = ['M', 'S'];
    let directions_cross = [(1, 1), (1, -1)];

    let mut straight_result = 0;
    let mut cross_result = 0;

    for (i_line, line) in input.iter().enumerate() {
        for (i_col, &c) in line.iter().enumerate() {
            if c == 'A' {
                let mut diagonal_count = 0;
                for dir in directions_cross.iter() {

                    if match_cross_pattern(&input, &pattern_cross, i_line as i32, i_col as i32, *dir) {
                        diagonal_count += 1;
                    }
                }
                if diagonal_count == 2 {
                    cross_result += 1;
                }
            }

            for dir in directions_straight.iter() {
                if matches_straight_pattern(&input, &pattern_straight, i_line as i32, i_col as i32, *dir) {
                    straight_result += 1;
                }
            }
        }
    }

    println!("straight patterns: {}\ncross patterns: {}", straight_result, cross_result);
}

fn matches_straight_pattern(
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

fn match_cross_pattern(
    input: &[Vec<char>],
    pattern: &[char],
    start_x: i32,
    start_y: i32,
    dir: (i32, i32),
) -> bool {
    let x_1 = start_x + dir.0;
    let y_1 = start_y + dir.1;
    let x_2 = start_x - dir.0;
    let y_2 = start_y - dir.1;

    if x_1 < 0 || y_1 < 0 || x_2 < 0 || y_2 < 0
        || x_1 >= input.len() as i32 || y_1 >= input[0].len() as i32
        || x_2 >= input.len() as i32 || y_2 >= input[0].len() as i32
    {
        return false;
    }

    let c1 = input[x_1 as usize][y_1 as usize];
    let c2 = input[x_2 as usize][y_2 as usize];

    c1 != c2 && pattern.contains(&c1) && pattern.contains(&c2)
}
