use std::{fs::File, io::{BufReader, BufRead}};


pub fn solve() -> usize {
    let priorities = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
                                  "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

    let file = File::open("src/day3/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut groups = vec![];
    let mut current_group = vec![];
    for line in reader.lines() {
        current_group.push(line.unwrap());
        if current_group.len() == 3 {
            let current_group_clone = current_group.clone();
            groups.push(current_group_clone);
            current_group.clear();
        }
    }
    
    let mut total = 0;
    for group in groups {
        for item in group.get(0).unwrap().chars() {
            if group.get(1).unwrap().contains(item) && group.get(2).unwrap().contains(item) {
                total += priorities.iter().position(|&r| r == item.to_string()).unwrap() + 1;
                break
            }
        }
    }
    total
}