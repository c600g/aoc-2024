use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf = BufReader::new(file);
    let mut s = String::new();
    let mut rules: Vec<(u8,u8)> = Vec::new();
    let mut parse_rules = true;
    let mut sum = 0_u32;
    let mut sum_fixed = 0_u32;

    loop {
        let count = buf.read_line(&mut s).unwrap();
        if count > 0 {
            // if we are parsing rules
            if parse_rules {
                match s.find('|') {
                    None => parse_rules = false,
                    Some(pos) => rules.push((s[..pos].trim().parse().unwrap(),s[pos+1..].trim().parse().unwrap())),
                }
            } 
            // otherwise, we are now parsing pages
            else {
                // if this is an empty line, then continue on
                s = s.trim().to_string();
                // if s == "" {
                //     continue;
                // }
                let pages: Vec<u8> = s.split(',').map(|i| i.parse::<u8>()).map(Result::unwrap).collect();
                sum += check_pages(&pages, &rules) as u32;
                sum_fixed += fix_pages(&pages, &rules) as u32;
            }
        }
        else {
            break;    
        }
        s.clear();
    }
    println!("PART 1: Sum of good middle pages: {}", sum);
    println!("PART 2: Sum of fixed middle pages: {}", sum_fixed);
}

fn check_pages(pages: &Vec<u8>, rules: &Vec<(u8,u8)>) -> u8 {
    let mut ordered = true;
    let mut middle = 0_u8;

    // loop through all of our rules
    for rule in rules {
        // if the pages listed in the rule are both in the pages array
        if let Some(i) = pages.iter().position(|&r| r == rule.0) {
            if let Some(j) = pages.iter().position(|&r| r == rule.1) {
                if j < i {
                    ordered = false;
                    break;
                }
            }
        }
    }
    if ordered {
        middle = pages[pages.len() / 2];
    }
    middle
}

fn fix_pages(pages: &Vec<u8>, rules: &Vec<(u8,u8)>) -> u8 {
    let mut ordered = true;
    let mut middle = 0_u8;

    // loop through all of our rules
    for rule in rules {
        if let Some(i) = pages.iter().position(|&r| r == rule.0) {
            if let Some(j) = pages.iter().position(|&r| r == rule.1) {
                if j < i {
                    ordered = false;
                    break;
                }
            }
        }
    }
    let mut fixed_pages: Vec<u8> = pages.clone();
    while !ordered {
        // fix the ordering here!
        ordered = true;
        for rule in rules {
            // if the pages listed in the rule are both in the pages array
            if let Some(i) = fixed_pages.iter().position(|&r| r == rule.0) {
                if let Some(j) = fixed_pages.iter().position(|&r| r == rule.1) {
                    if j < i {
                        //then fix it here!
                        let temp = fixed_pages[j];
                        fixed_pages[j] = fixed_pages[i];
                        fixed_pages[i] = temp;
                        ordered = false;
                        break;
                    }
                }
            }
        }
        middle = fixed_pages[fixed_pages.len() / 2];
    }
    middle
}