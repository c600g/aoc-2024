use std::fs::File;
use std::num;
use std::io::{self, BufRead, BufReader};
fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf = BufReader::new(file);
    let mut s = String::new();
    let mut vec1: Vec<i64> = Vec::new();
    let mut vec2: Vec<i64> = Vec::new();
    loop {
        let count = buf.read_line(&mut s).unwrap();
        if count > 0 {
            let mut iter = s.split_whitespace();
            if let Some(v1) = iter.next() {
                vec1.push(v1.parse().unwrap());
            }
            if let Some(v2) = iter.next() {
                vec2.push(v2.parse().unwrap());
            }
        }
        else {
            break;    
        }
        s.clear();
    }
    vec1.sort();
    vec2.sort();
    
    // compute the distance between the lists
    let mut distance = 0_i64;
    for i in 0..vec1.len() {
        distance = distance + (vec1[i] - vec2[i]).abs();
    }
    println!("Distance is: {distance}");

    // compute the similarity score
    let mut similarity= 0_i64;
    let mut last = 0_i64;
    let mut count = 0_64;
    for v1 in vec1 {
        if v1 == last {
            similarity = similarity + last * count;
            continue;
        }
        // ok, we have a new v1 - how many times does it appear in v2?
        last = v1;
        count = 0;
        for v2 in &vec2 {
            if v2 > &v1 { break; }
            if v2 == &v1 { count += 1; }
        }
        similarity = similarity + last * count;
    }
    println!("Similarity is: {similarity}");
}

