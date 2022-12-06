
pub fn solve() -> usize {
    let stream = include_str!("input.txt").to_string();
    
    let mut buffer = Vec::new();
    let mut count = 0;
    for c in stream.chars() {
        count += 1;
        buffer.insert(0, c.to_string());

        if buffer.len() == 15 {
            buffer.remove(14);
        }

        if buffer.len() == 14 && !check_duplicates(&buffer) {
            break;
        }
    }
    count
}

fn check_duplicates(list: &Vec<String>) -> bool {
    let mut buffer = Vec::new();

    for c in list {
        if buffer.contains(&c) {
            return true
        }
        buffer.push(c);
    }
    false
}