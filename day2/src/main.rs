use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn try_made_safe_levels(report: &Vec<i32>) -> bool {
    for idx in 0..report.len() {
        let mut updated_report = report.clone();
        updated_report.remove(idx);

        if is_safe_levels(&updated_report) {
            return true;
        }
    }

    false
}

fn is_safe_levels(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;
    for idx in 0..report.len() - 1 {
        let diff = report[idx + 1] - report[idx];
        if diff < -3 || diff == 0 || diff > 3 {
            return false;
        }

        if diff > 0 {
            decreasing = false;
        }

        if diff < 0 {
            increasing = false;
        }
    }

    increasing || decreasing
}

fn verify_raports(filename: &str) -> i32 {
    let file = File::open(filename)
        .map_err(|e| format!("Cannot open file: {}", e))
        .unwrap();
    let reader = BufReader::new(file);
    let result = reader
        .lines()
        .map(|line| match line {
            Ok(line) => line
                .split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok())
                .collect::<Vec<_>>(),
            Err(_) => todo!(),
        })
        .filter(|report| is_safe_levels(report) || try_made_safe_levels(report))
        .count();

    result.try_into().unwrap()
}

fn main() {
    let result = verify_raports("./input.txt");
    println!("result: {}", result);
}
