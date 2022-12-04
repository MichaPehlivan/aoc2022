use std::{fs::File, io::{BufReader, BufRead}};


pub fn solve() -> usize {
    let file = File::open("src/day4/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let pair = line.unwrap();
        let sections = pair.split(",").collect::<Vec<&str>>();
        let range1 = sections.get(0).unwrap().split("-").collect::<Vec<&str>>();
        let range2 = sections.get(1).unwrap().split("-").collect::<Vec<&str>>();

        let a = usize::from_str_radix(range1.get(0).unwrap(), 10).unwrap();
        let b = usize::from_str_radix(range1.get(1).unwrap(), 10).unwrap();
        let c = usize::from_str_radix(range2.get(0).unwrap(), 10).unwrap();
        let d = usize::from_str_radix(range2.get(1).unwrap(), 10).unwrap();

        if a <= d && c <= b{
            total += 1;
        }
    }

    total
}