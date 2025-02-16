use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

const WORD: &[char] = &['X', 'M', 'A', 'S'];
const DIRECTIONS: &[(isize, isize)] = &[
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn search(puzzle: &Vec<Vec<char>>, word: &[char]) -> usize {
    let rows = puzzle.len();
    let cols = puzzle.len();
    let word_len = word.len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in DIRECTIONS {
                let mut found = true;
                for i in 0..word_len {
                    let nr = r as isize + i as isize * dr;
                    let nc = c as isize + i as isize * dc;

                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        found = false;
                        break;
                    }

                    if puzzle[nr as usize][nc as usize] != word[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }
    count
}

fn read_input(filename: &str) -> Result<Vec<Vec<char>>, Error> {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .flat_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    Ok(grid)
}

fn main() {
    let data = read_input("./input.txt");
    match data {
        Ok(d) => {
            let result = search(&d, WORD);
            println!("Result: {}", result);
        }
        Err(e) => eprintln!("{}", e),
    }
}
