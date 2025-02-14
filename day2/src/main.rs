use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type RedNosedReports = Vec<Vec<u32>>;

fn verify_report(reports: RedNosedReports) -> u32 {
    if reports.len() == 0 {
        return 0;
    }
    println!("Amount of reports to check: {}", reports.len());

    // let mut stack: Vec<u32> = vec![];
    for report in reports {
        println!("{:?}", report);
    }
    0
}

fn read_input(filename: &str) -> Result<RedNosedReports, Box<dyn std::error::Error>> {
    let file = File::open(filename).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);
    let mut reports: RedNosedReports = vec![];

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let parts = content.split(" ");
                let collection: Result<Vec<u32>, _> = parts
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|x| x.parse::<u32>())
                    .collect();

                let report = collection.map_err(|e| format!("Failed to convert &str to u32: {}", e))?;
                reports.push(report);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(reports)
}

fn main() {
    match read_input("./input.txt") {
        Ok(data) => {
            let result = verify_report(data);
            println!("Amout of safe reports: {}", result);
        }
        Err(e) => eprintln!("{}", e),
    }
}
