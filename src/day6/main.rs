use std::{fs::File, io::{BufRead, BufReader}, sync::Mutex, rc::Rc};

fn main() {
    let filepath = "C:\\users\\marcu\\projects\\adventofcode24\\src\\input.txt";
    let file = File::open(filepath).expect("Failed to open file");
    let reader = BufReader::new(file);
    let map: Rc<Mutex<Vec<Vec<char>>>> = Rc::from(Mutex::new(Vec::new()));
    let mut result: i32 = 0;
    let mut guard = Guard::new(Rc::clone(&map));

    for (i,line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line").chars().collect::<Vec<_>>();
        if let Some(x) = line.iter().position(|c| *c == '^') {
            guard.x_pos = x as i32;
            guard.y_pos = i as i32;
           
        }
        map.lock().unwrap().push(line);
        
    }
    //über ne fucking stunde verschwendet mit fehlersuche, weil ich idiot das erste Feld nicht als besucht markiert hab...
    guard.map.lock().unwrap()[guard.y_pos as usize][guard.x_pos as usize] = 'X';
    println!("start x_pos: {}\nstart y_pos: {}\nstart dir: {:?}", guard.x_pos, guard.y_pos, guard.directions[guard.curr_dir]);
    while guard.step() {
    }
    for row in map.lock().unwrap().iter() {
        for c in row {
            if *c == 'X' {
                result += 1
            }
        }
    }

    
    println!("result: {}", result);
}

 struct Guard {
    x_pos: i32,
    y_pos: i32,
    directions: [(i32, i32);4],
    curr_dir: usize,
    stepsize: i32,
    map: Rc<Mutex<Vec<Vec<char>>>>,
 }
 impl Guard {
    pub fn new(map: Rc<Mutex<Vec<Vec<char>>>>) -> Guard {
        Guard { x_pos: 0, y_pos: 0, directions: [(0,-1),(1,0),(0,1),(-1,0)], curr_dir: 0, stepsize: 1, map: map }
    }
    pub fn rotate(&mut self) {
        self.curr_dir += 1;
        if self.curr_dir > 3 {
            self.curr_dir = 0;
        }
    }
    fn step(&mut self) -> bool {
        let new_x = self.x_pos + self.stepsize * self.directions[self.curr_dir].0;
        let new_y = self.y_pos + self.stepsize * self.directions[self.curr_dir].1;
        if self.is_within_bounds(new_x, new_y) {
            if self.map.lock().unwrap()[new_y as usize][new_x as usize] == '#' {
                // println!("x_pos: {}\ny_pos: {}", self.x_pos, self.y_pos);
                self.rotate();
            } else {
                self.x_pos = new_x;
                self.y_pos = new_y;
                self.map.lock().unwrap()[self.y_pos as usize][self.x_pos as usize] = 'X';
            }
            true        
        } else {
            println!("final x_pos: {}\nfinal y_pos: {}\nfinal dir: {:?}", self.x_pos, self.y_pos, self.directions[self.curr_dir]);
            false
        }
    }

    fn is_within_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (y as usize) < self.map.lock().unwrap().len() && (x as usize) < self.map.lock().unwrap()[0].len()
    }
 }