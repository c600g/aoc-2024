use std::fs::File;
use std::io::{BufRead, BufReader};

struct Map {
    chars: Vec<char>,
    cols: usize,
    rows: usize,
}

impl Map {
    fn new() -> Self {
        Self {
            chars: Vec::new(),
            cols: 0,
            rows: 0,
        }
    }

    fn from_file(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let mut buf = BufReader::new(file);
        //let mut s = String::new();
        let mut chars = Vec::new();
        let mut cols = 0;
        let mut rows = 0;

        loop {
            let mut s = String::new();
            let count = buf.read_line(&mut s).unwrap();
            s = s.trim().to_string();
            if count > 0 && s.len() > 0 {
                cols = s.len();
                let mut string_chars: Vec<char> = s.chars().collect();
                chars.append(&mut string_chars);
                rows += 1;
            } else {
                break;
            }
        }
        Self { chars, cols, rows }
    }

    fn from_cols_rows(cols: usize, rows: usize) -> Self {
        let mut chars: Vec<char> = Vec::new();
        for _i in 0..cols * rows {
            chars.push('.');
        }
        Self { chars, cols, rows }
    }

    fn print(&self) {
        for (i, c) in self.chars.iter().enumerate() {
            if i % self.cols == 0 && i != 0 {
                print!("\n");
            }
            print!("{}", c);
        }
        println!("");
    }

    fn col_row_from_index(&self, index: usize) -> (usize, usize) {
        (index % self.cols, index / self.cols)
    }

    fn index_from_col_row(&self, col: usize, row: usize) -> usize {
        row * self.cols + col
    }

    fn find_all(&self, search: char) -> Vec<usize> {
        let mut results = Vec::new();
        for (i, c) in self.chars.iter().enumerate() {
            if *c == search {
                results.push(i);
            }
        }
        results
    }
}

fn main() {
    let topo = Map::from_file("input.txt");
    println!("");
    println!(
        "Topographical map is {} cols by {} rows.",
        topo.cols, topo.rows
    );
    topo.print();
    println!("");
    // find all trailheads
    let trailheads = topo.find_all('0');
    println!("Trailheads found: {:?}", trailheads);
    // find all summit trails for each trailhed
    let mut total_summit_trails = 0;
    for (i, th) in trailheads.iter().enumerate() {
        let mut summits = Vec::new();
        let result = find_trail_count(&topo, *th, &mut summits);
        total_summit_trails += result;
        //println!("Summit trails for trailhead #{}: {}", i, result);
    }
    println!("");
    println!("Part 1 - Total summit trails: {}", total_summit_trails);
    let mut total_trail_ratings = 0;
    for (i, th) in trailheads.iter().enumerate() {
        let result = find_trail_rating(&topo, *th);
        total_trail_ratings += result;
        //println!("Trail rating for trailhead #{}: {}", i, result);
    }
    println!("");
    println!("Part 2 - Total trail ratings: {}", total_trail_ratings);
}

fn find_trail_count(topo: &Map, loc: usize, summits: &mut Vec<usize>) -> usize {
    let mut result = 0;
    let elev = topo.chars[loc].to_digit(10).unwrap() as usize;
    //let (col, row) = topo.col_row_from_index(loc);
    // if we have reacged a peak, indicate success!
    if elev == 9 {
        if summits.iter().any(|&x| x == loc) {
            //println!("{}loc: {} ({},{}) elev: {} -> ALREADY VISITED", "-".repeat(elev), loc, col, row, elev);
            return 0;
        } else {
            //println!("{}loc: {} ({},{}) elev: {} -> SUMMIT", "-".repeat(elev), loc, col, row, elev);
            summits.push(loc);
            return 1;
        }
    }
    let paths = find_next_in_trail(topo, loc);
    //println!("{}loc: {} ({},{}) elev: {} -> {:?}", "-".repeat(elev), loc, col, row, elev, paths);
    for path in paths {
        result += find_trail_count(topo, path, summits);
    }
    result
}

fn find_trail_rating(topo: &Map, loc: usize) -> usize {
    let mut result = 0;
    let elev = topo.chars[loc].to_digit(10).unwrap() as usize;
    //let (col, row) = topo.col_row_from_index(loc);
    // if we have reacged a peak, indicate success!
    if elev == 9 {
        //println!("{}loc: {} ({},{}) elev: {} -> SUMMIT", "-".repeat(elev), loc, col, row, elev);
        return 1;
    }
    let paths = find_next_in_trail(topo, loc);
    //println!("{}loc: {} ({},{}) elev: {} -> {:?}", "-".repeat(elev), loc, col, row, elev, paths);
    for path in paths {
        result += find_trail_rating(topo, path);
    }
    result
}

// given a location on our map, return a vector of valid
// locations which are legal for the next location.
fn find_next_in_trail(topo: &Map, loc: usize) -> Vec<usize> {
    let elev: i8 = topo.chars[loc].to_digit(10).unwrap() as i8;
    let mut results = Vec::new();

    // if we are at a peak, we are done!
    if elev == 9 {
        return results;
    }

    // otherwise, we need to look for valid paths for this location
    let (col, row) = topo.col_row_from_index(loc);
    // check up
    if row > 0 {
        let next = loc - topo.cols;
        let next_elev: i8 = topo.chars[next].to_digit(10).unwrap() as i8;
        if next_elev - elev == 1 {
            results.push(next);
        }
    }
    // check right
    if col < topo.cols - 1 {
        let next = loc + 1;
        let next_elev: i8 = topo.chars[next].to_digit(10).unwrap() as i8;
        if next_elev - elev == 1 {
            results.push(next);
        }
    }
    // check down
    if row < topo.rows - 1 {
        let next = loc + topo.cols;
        let next_elev: i8 = topo.chars[next].to_digit(10).unwrap() as i8;
        if next_elev - elev == 1 {
            results.push(next);
        }
    }
    // check left
    if col > 0 {
        let next = loc - 1;
        let next_elev: i8 = topo.chars[next].to_digit(10).unwrap() as i8;
        if next_elev - elev == 1 {
            results.push(next);
        }
    }
    results
}
