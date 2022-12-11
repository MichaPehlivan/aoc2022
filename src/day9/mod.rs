use std::{fs::File, io::{BufReader, BufRead}};


pub fn solve() -> usize {
    let file = File::open("src/day9/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut knots = vec![(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0), (0, 0)];
    let mut pos_visited: Vec<(i32, i32)> = vec![(0, 0)];
    for line in reader.lines() {
        let line = line.unwrap();
        let move_dir = line.chars().next().unwrap().to_string();
        let steps = i32::from_str_radix(&line[2..], 10).unwrap();

        for _ in 0..steps {
            match move_dir.as_str() {
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                "L" => knots[0].0 -= 1,
                "R" => knots[0].0 += 1,
                _=> panic!("invalid direction")
            }
            move_knot(knots[0], &mut knots[1], &mut pos_visited, false);
            move_knot(knots[1], &mut knots[2], &mut pos_visited, false);
            move_knot(knots[2], &mut knots[3], &mut pos_visited, false);
            move_knot(knots[3], &mut knots[4], &mut pos_visited, false);
            move_knot(knots[4], &mut knots[5], &mut pos_visited, false);
            move_knot(knots[5], &mut knots[6], &mut pos_visited, false);
            move_knot(knots[6], &mut knots[7], &mut pos_visited, false);
            move_knot(knots[7], &mut knots[8], &mut pos_visited, false);
            move_knot(knots[8], &mut knots[9], &mut pos_visited, true);
        }
    }
    remove_duplicates(pos_visited).len()
}

fn move_knot(prev: (i32, i32), knot: &mut (i32, i32), visited: &mut Vec<(i32, i32)>, recording: bool) {
    if i32::abs_diff(prev.0, knot.0) > 1 || i32::abs_diff(prev.1, knot.1) > 1 {
        if prev.0 == knot.0 {
            if prev.1 > knot.1 {
                knot.1 += 1;
            }
            else {
                knot.1 -= 1;
            }
        }
        else if prev.1 == knot.1 {
            if prev.0 > knot.0 {
                knot.0 += 1;
            }
            else {
                knot.0 -= 1;
            }
        }
        else if prev.0 > knot.0 {
            if prev.1 > knot.1 {
                knot.0 += 1;
                knot.1 += 1;
            }
            else {
                knot.0 += 1;
                knot.1 -= 1;
            }
        }
        else {
            if prev.1 > knot.1 {
                knot.0 -= 1;
                knot.1 += 1;
            }
            else {
                knot.0 -= 1;
                knot.1 -= 1;
            }
        }
        
        if recording {
            visited.push(*knot);
        }
    }
}

fn remove_duplicates(vector: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut removed = Vec::new();

    for item in vector.iter() {
        if !removed.contains(item) {
            removed.push(*item);
        }
    }
    removed
}
