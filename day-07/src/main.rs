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
            let sum: usize = (&s[..colon].trim()).to_string().parse().expect("Can not parse.");
            let args = (&s[colon + 1..].trim()).to_string();
            let args: Vec<usize> = args.split_whitespace().filter_map(|s| s.parse().ok()).collect();
            println!("sum:{} args: {:?}", sum, args);
        } else {
            break;
        }
        s.clear();
    }

}
