use std::{fs::File, io::Read};
fn main() {
    let filepath = "C:\\users\\marcu\\projects\\adventofcode24\\src\\input.txt";
    let mut file = File::open(filepath).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);
    let chars: Vec<char> = input.chars().collect();
    let keyword_mul: Vec<char> = "mul(".chars().collect();
    let keyword_do: Vec<char> = "do()".chars().collect();
    let keyword_dont: Vec<char> = "don't()".chars().collect();
    let mut mul_enabled: bool = true;
    let mut result: Vec<i32> = Vec::new();
    for (i,w) in chars.windows(keyword_dont.len()).enumerate() {
        if w[..keyword_mul.len()] == keyword_mul && mul_enabled == true {
            match parse_arguments(&chars[i+keyword_mul.len()..]) {
                Ok(args) => result.push(args.iter().product()),
                Err(e) => eprintln!("{}",e),
            };
        } else if w[..keyword_do.len()] == keyword_do {
            mul_enabled = true;
        } else if w == keyword_dont {
            mul_enabled = false
        }
    }
    println!("{:?}", result.iter().sum::<i32>())
}

fn parse_arguments(args: &[char]) -> Result<Vec<i32>, String> {
    let mut result = Vec::new();
    let mut arg = String::new();
    let mut chars_iter = args.iter();

    while let Some(&c) = chars_iter.next() {
        if c.is_ascii_digit() {
            arg.push(c);
        } else if c == ',' || c == ')' {
            if !arg.is_empty() {
                result.push(arg.parse::<i32>().map_err(|_| format!("Failed to parse number: {}", arg))?);
                arg.clear();
            }
            if c == ')' {
                return Ok(result);
            }
        } else if !c.is_whitespace() {
            return Err(format!("Unexpected character: {}", c));
        }
    }

    Err("Arguments did not terminate with ')'".to_string())
}
