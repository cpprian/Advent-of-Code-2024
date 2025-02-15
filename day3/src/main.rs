use std::fs;

use regex::Regex;

fn fix_memory_corruption(memory: String) -> u32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((?<x>[0-9]*),(?<y>[0-9]*)\)").unwrap();
    let caps = re.captures_iter(&memory);

    let mut do_mul = true;
    let mut result = 0;
    for cap in caps {
        let task = cap.get(0).map_or("", |m| m.as_str());
        match task {
            "do()" => { do_mul = true; },
            "don't()" => { do_mul = false; },
            _ => {
                if do_mul {
                    let x = &cap["x"].parse::<u32>().unwrap();
                    let y = &cap["y"].parse::<u32>().unwrap();
                    result += x * y;
                }
            }
        }
    }

    result
}

fn read_input(filename: &str) -> Result<String, String> {
    fs::read_to_string(filename).map_err(|e| format!("Cannot read {}: {}", filename, e))
}

fn main() {
    let data = read_input("./input.txt");
    match data {
        Ok(mem) => {
            let result = fix_memory_corruption(mem);
            println!("Result: {}", result);
        }
        Err(e) => eprintln!("{}", e),
    }
}
