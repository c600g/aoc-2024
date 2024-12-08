use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("test_input.txt").unwrap();
    let mut buf = BufReader::new(file);
    let mut s = String::new();

    loop {
        let count = buf.read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if count > 0 && s.len() > 0 {
            let colon = s.find(":").unwrap();
            let sum = &s[..colon - 1].trim().to_string();
            let args = &s[colon + 1..].trim().to_string();
            let args: Vec<String> = args.split_whitespace().collect();
            println!("sum:{} args: {:?}", sum, args);
        } else {
            break;
        }
        s.clear();
    }

}
