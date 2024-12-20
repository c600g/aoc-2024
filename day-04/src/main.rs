use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);
    let lines: Vec<_> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let mut matches = 0;
    let word = "XMAS";

    // loop through each line
    for (y, line) in lines.iter().enumerate() {
        // loop through each character on the line
        for x in 0..line.len() {
            matches += is_match(x, y, 1, 0, word, &lines);
            matches += is_match(x, y, -1, 0, word, &lines);
            matches += is_match(x, y, 0, 1, word, &lines);
            matches += is_match(x, y, 0, -1, word, &lines);
            matches += is_match(x, y, 1, 1, word, &lines);
            matches += is_match(x, y, 1, -1, word, &lines);
            matches += is_match(x, y, -1, 1, word, &lines);
            matches += is_match(x, y, -1, -1, word, &lines);
        }
    }
    println!("Part 1 matches: {}", matches);

    // Part 2 - we need to search for X-MAS
    matches = x_match(&lines);
    println!("Part 2 matches: {}", matches);
}

fn is_match(x: usize, y: usize, x_dir: i8, y_dir: i8, word: &str, lines: &Vec<String>) -> i32 {
    // do some bounds checking first
    if x_dir > 0 && x + word.len() - 1 >= lines[y].len() {
        return 0;
    }
    if x_dir < 0 && x < word.len() - 1 {
        return 0;
    }
    if y_dir > 0 && y + word.len() -1 >= lines.len() {
        return 0;
    }
    if y_dir < 0 && y < word.len() - 1 {
        return 0;
    }

    let mut good_match = true;
    let mut cur_x = x;
    let mut cur_y = y;
    // okay, if we get this far then we are good!
    for c in word.chars() {
        if c != lines[cur_y].chars().nth(cur_x).unwrap() {
            good_match = false;
            break;
        }
        if x_dir > 0 {
            cur_x += 1;
        }
        if x_dir < 0 && cur_x > 0 {
            cur_x -= 1;
        }
        if y_dir > 0 {
            cur_y += 1;
        }
        if y_dir < 0 && cur_y > 0 {
            cur_y -= 1;
        }
    }
    if good_match {
        1
    } else {
        0
    }
}

fn x_match(lines: &Vec<String>) -> i32 {
    let mut matches = 0;

    // loop through each line
    for (y, line) in lines.iter().enumerate() {
        // if this is the first or last line, no checks are required
        if y == 0 || y == lines.len() - 1 {
            continue;
        }
        // loop through each character on the line
        for x in 0..line.len() {
            // if this is the first or last character, no checks are required
            if x == 0 || x == line.len() -1 {
                continue;
            }
            // build up first test
            let test1 = format!("{}{}{}", lines[y-1].chars().nth(x-1).unwrap(), lines[y].chars().nth(x).unwrap(), lines[y+1].chars().nth(x+1).unwrap());
            let test2 = format!("{}{}{}", lines[y-1].chars().nth(x+1).unwrap(), lines[y].chars().nth(x).unwrap(), lines[y+1].chars().nth(x-1).unwrap());
            if ( test1 == "SAM" || test1 == "MAS" ) &&  ( test2 == "SAM" || test2 == "MAS" ) {
                matches += 1;
            }
        }
    }
    matches
}