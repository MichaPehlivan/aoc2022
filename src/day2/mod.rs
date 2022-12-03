use std::{fs::File, io::{BufReader, BufRead}};


pub fn solve() -> usize {
    let file = File::open("src/day2/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line_move = line.unwrap();
        let moves: Vec<&str> = line_move.trim().split_ascii_whitespace().collect();
        total += get_score(moves.get(0).unwrap(), moves.get(1).unwrap());
    }

    total
}

fn get_score(move1: &str, result: &str) -> usize {
    let mut score = 0;
    score += match result {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _=> panic!("invalid result")
    };

    score += match (move1, result) {
        ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
        ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
        ("A", "X") | ("B", "Z") | ("C", "Y") => 3,
        _=> panic!("invalid move or result")
    };
    score
}