use std::{fs::File, io::{BufReader, BufRead}};


pub fn solve() -> String {
    let file = File::open("src/day5/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut boxes: [Vec<String> ; 9] = Default::default();

    for line in &mut reader.lines() {
        let line = line.unwrap();

        if line.contains("[") {
            let line = line + " ";
            for i in 0..9 {
                let box_str = line[i*4..i*4+4].trim();

                if !box_str.is_empty() {
                    let character = box_str.replace("[", "").replace("]", "");
                    boxes[i].push(character);
                }
            }
        }
        else if line.contains("move") {
            let line: Vec<&str> = line.split(" ").collect();
            let moves = usize::from_str_radix(line.get(1).unwrap().trim(), 10).unwrap();
            let from = usize::from_str_radix(line.get(3).unwrap().trim(), 10).unwrap();
            let to = usize::from_str_radix(line.get(5).unwrap().trim(), 10).unwrap();

            let mut buffer = Vec::new();
            {
                let from_vec = &mut boxes[from-1];

                for _ in 0..moves {
                    let removed = from_vec.remove(0);
                    buffer.insert(0, removed);
                }
            }
            let to_vec = &mut boxes[to-1];
            
            for item in buffer {
                to_vec.insert(0, item)
            }
        }
    }

    let mut result = String::new();
    for i in 0..9 {
        let on_top = boxes[i].get(0).unwrap();
        result.push_str(on_top);
    }

    result
}