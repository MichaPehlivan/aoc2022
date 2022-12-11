use std::{fs::File, io::{BufReader, BufRead}};


pub fn solve() -> String {
    let file = File::open("src/day10/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut x = 1;
    let mut cycles = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        if line == "noop" {
            cycles.push(0);
        }
        else {
            let cmd: Vec<&str> = line.split(" ").collect();
            cycles.push(0);
            let added = i32::from_str_radix(cmd.get(1).unwrap(), 10).unwrap();
            cycles.push(added);
        }
    }

    let mut cycle = 0;
    let mut image = String::from("\n");
    let mut spirite_pos: [i32; 3];
    for i in cycles {
        cycle += 1;

        spirite_pos = [x-1, x, x+1];
        let drawing = cycle % 40 - 1;
        if spirite_pos.contains(&drawing) {
            image.push_str("#");
        }
        else {
            image.push_str(".");
        }

        if cycle == 40 || cycle == 80 || cycle == 120 || cycle == 160 || cycle == 200 || cycle == 240 {
            image.push_str("\n");
        } 

        x += i;
    }

    image
}