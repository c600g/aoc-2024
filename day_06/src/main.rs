use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("test_input.txt").unwrap();
    let mut buf = BufReader::new(file);
    let mut s = String::new();
    let mut map: Vec<char> = Vec::new();
    let mut cols = 0;
    let mut rows = 0;

    loop {
        let count = buf.read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if count > 0 && s.len() > 0 {
            cols = s.len();
            let mut chars: Vec<char> = s.chars().collect();
            map.append(&mut chars);
            rows += 1;
        } else {
            break;
        }
        s.clear();
    }
    println!("Map is {} cols by {} rows.", cols, rows);
}
