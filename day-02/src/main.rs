use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf = BufReader::new(file);
    let mut report = String::new();
    let mut safe_count = 0_i32;
    let mut sorta_safe_count = 0_i32;

    loop {
        let count = buf.read_line(&mut report).unwrap();
        if count > 0 {
            if report_is_safe(&report) {
                safe_count += 1;
                sorta_safe_count += 1;
            } else if report_is_sorta_safe(&report) {
                sorta_safe_count += 1;
            }
        }
        else {
            break;    
        }
        report.clear();
    }

    println!("Safe report count: {safe_count}");
    println!("Sorta safe report count: {sorta_safe_count}");
}

fn report_is_safe(report: &String) -> bool {
    let s = String::from(report);
    let iter = s.split_whitespace();
    let mut levels: Vec<i8> = Vec::new();
    for val in iter {
        let level: i8 = val.parse().unwrap();
        levels.push(level);
    }
    levels_are_safe(&levels)
}

fn levels_are_safe(levels: &Vec<i8>) -> bool {
    // now that we have all of our levels parsed and
    // loaded into the levels vector, we can analyze them!
    let mut monotonic = true;
    let mut diff_ok = true;
    for i in 1..levels.len() {
        // check that adjacent levels differ by >= 1 and <= 3
        let diff = levels[i] - levels[i - 1];
        diff_ok = diff_ok && (diff.abs() >= 1) && (diff.abs() <= 3);
        if i > 1 {
            monotonic = monotonic && ( (diff > 0) && (levels[i - 1] - levels[i - 2] > 0) || (diff < 0) && (levels[i - 1] - levels[i - 2] < 0) )
        }
    }
    diff_ok && monotonic
}

fn report_is_sorta_safe(report: &String) -> bool {
    let s = String::from(report);
    let iter = s.split_whitespace();
    let mut levels: Vec<i8> = Vec::new();
    for val in iter {
        let level: i8 = val.parse().unwrap();
        levels.push(level);
    }
    
    let mut sorta_safe = false;
    for i in 0..levels.len() {
        let mut small = levels.clone();
        small.remove(i);
        sorta_safe = sorta_safe || levels_are_safe(&small)
    }
    sorta_safe
}