use std::{fs::File, io::{BufReader, BufRead}};


pub fn solve() -> usize {
    let file = File::open("src/day8/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut trees = [[0; 99]; 99];
    let mut row = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        
        let mut collumn = 0;
        for c in line.chars() {
            let height = usize::from_str_radix(&c.to_string(), 10).unwrap();
            trees[row][collumn] = height;
            collumn += 1;
        }
        row += 1;
    }
    
    let mut scores: Vec<usize> = Vec::new();
    for x in 0..99 {
        for y in 0..99 {
            let height = trees[y][x];
            let mut visible_left = 0;
            let mut visible_right = 0;
            let mut visible_top = 0;
            let mut visible_bottom = 0;

            for i in (0..x).rev() {
                if trees[y][i] >= height {
                    visible_left += 1;
                    break;
                }
                if trees[y][i] < height {
                    visible_left += 1;
                }
            }
            for i in (x+1)..99 {
                if trees[y][i] >= height {
                    visible_right += 1;
                    break;
                }
                if trees[y][i] < height {
                    visible_right += 1;
                }
            }
            for i in (0..y).rev() {
                if trees[i][x] >= height {
                    visible_top += 1;
                    break;
                }
                if trees[i][x] < height {
                    visible_top += 1;
                }
            }
            for i in (y+1)..99 {
                if trees[i][x] >= height {
                    visible_bottom += 1;
                    break;
                }
                if trees[i][x] < height {
                    visible_bottom += 1;
                }
            }

            let score = visible_left * visible_right * visible_top * visible_bottom;
            scores.push(score);
        }
    }
    
    let mut greatest: usize = usize::from(*scores.get(0).unwrap());
    for score in scores.iter() {
        if *score > greatest {
            greatest = *score;
        }
    }
    greatest
}
