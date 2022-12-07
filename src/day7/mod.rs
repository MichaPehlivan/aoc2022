use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};


pub fn solve() -> usize {
    let file = File::open("src/day7/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut current_extension = String::new();
    let mut parents = Vec::new();
    let mut files: HashMap<String, usize> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with("$ cd") {
            if &line[5..] == ".." {
                parents.remove(parents.len()-1);
                let new_path = current_extension[..current_extension.rfind("/").unwrap()].to_string();
                current_extension = new_path;
            }
            else {
                current_extension.push_str(&format!("{}{}", "/", &line[5..]));
                parents.push(current_extension.clone());
            }
        }

        if line.chars().next().unwrap().is_digit(10) {
            let mut size = String::new();
            for c in line.chars() {
                if c == ' ' {
                    break;
                }
                size.push(c);
            }
            for ext in &parents {
                let old_size = if let Some(size) = files.get(&ext.clone()) {size} else {&0};
                files.insert(ext.clone(), old_size + usize::from_str_radix(&size, 10).unwrap());
            }
        }
    }
    
    let total_used = files.get("//").unwrap();
    let space_needed = 30000000 - (70000000 - total_used);

    let mut possible_dirs = Vec::new();
    for dir in files.iter() {
        if *dir.1 >= space_needed {
            possible_dirs.push(*dir.1);
        }
    }
    
    let mut lowest = *possible_dirs.get(0).unwrap();
    for dir in possible_dirs {
        if dir < lowest {
            lowest = dir;
        }
    }
    lowest
}
