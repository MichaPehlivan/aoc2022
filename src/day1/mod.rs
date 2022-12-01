use std::{fs::File, io::{prelude::*, BufReader}};

pub fn solve() -> usize {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut totals: Vec<usize> = vec![];
    let mut current: Vec<usize> = vec![];
    for line in reader.lines() {
        let line_clone = line.unwrap().clone();
        if line_clone.is_empty() {
            let mut sum = 0;
            for i in &current {
                sum += i;
            }
            totals.push(sum);
            current = Vec::new();
        }
        else {
            let n = usize::from_str_radix(line_clone.as_str(), 10).unwrap();
            current.push(n);
        }
    }
    
    let mut top3 = [0, 0, 0];
    for i in totals {
        if i >= top3[0] {
            top3[2] = top3[1];
            top3[1] = top3[0];
            top3[0] = i;
        }
        else if i >= top3[1] {
            top3[2] = top3[1];
            top3[1] = i;
        }
        else if i >= top3[2] {
            top3[2] = i;
        }
    }
    let total = top3[0] + top3[1] + top3[2];
    total
}