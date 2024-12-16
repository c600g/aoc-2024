use core::panic;
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

    fn find_first(&self, search: char) -> Option<usize> {
        for (i, c) in self.chars.iter().enumerate() {
            if *c == search {
                return Some(i);
            }
        }
        None
    }

    fn index_up(&self, index: usize) -> Option<usize> {
        if index >= self.cols && index < self.chars.len() {
            Some(index - self.cols)
        } else {
            None
        }
    }

    fn index_right(&self, index: usize) -> Option<usize> {
        if index % self.cols < self.cols - 1 && index < self.chars.len() - 1 {
            Some(index + 1)
        } else {
            None
        }
    }

    fn index_down(&self, index: usize) -> Option<usize> {
        if index < self.chars.len() - self.cols {
            Some(index + self.cols)
        } else {
            None
        }
    }

    fn index_left(&self, index: usize) -> Option<usize> {
        if index % self.cols > 0 {
            Some(index - 1)
        } else {
            None
        }
    }

    fn up(&self, index: usize, default: char) -> char {
        if let Some(index_up) = self.index_up(index) {
            self.chars[index_up]
        } else {
            default
        }
    }

    fn right(&self, index: usize, default: char) -> char {
        if let Some(index_rt) = self.index_right(index) {
            self.chars[index_rt]
        } else {
            default
        }
    }

    fn down(&self, index: usize, default: char) -> char {
        if let Some(index_dn) = self.index_down(index) {
            self.chars[index_dn]
        } else {
            default
        }
    }

    fn left(&self, index: usize, default: char) -> char {
        if let Some(index_lt) = self.index_left(index) {
            self.chars[index_lt]
        } else {
            default
        }
    }

    fn up_right(&self, index: usize, default: char) -> char {
        let mut result = default;
        if let Some(index_up) = self.index_up(index) {
            if let Some(index_up_right) = self.index_right(index_up) {
                result = self.chars[index_up_right];
            }
        }
        result
    }

    fn down_right(&self, index: usize, default: char) -> char {
        let mut result = default;
        if let Some(index_down) = self.index_down(index) {
            if let Some(index_down_right) = self.index_right(index_down) {
                result = self.chars[index_down_right];
            }
        }
        result
    }

    fn down_left(&self, index: usize, default: char) -> char {
        let mut result = default;
        if let Some(index_down) = self.index_down(index) {
            if let Some(index_down_left) = self.index_left(index_down) {
                result = self.chars[index_down_left];
            }
        }
        result
    }

    fn up_left(&self, index: usize, default: char) -> char {
        let mut result = default;
        if let Some(index_up) = self.index_up(index) {
            if let Some(index_up_left) = self.index_left(index_up) {
                result = self.chars[index_up_left];
            }
        }
        result
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
enum Facing {
    North,
    East,
    South,
    West
}

fn main() {
    part1();
    //part2();
}

fn part1() {
    let map = Map::from_file("test-input-1.txt");
    println!("Initial state:");
    map.print();
    println!("");
    find_paths(&map);
    println!("Part 1 minimum path count: {}", 0);
}

fn part2() {
    // now, for part 2 wherein the map is doubled!
    let map = Map::from_file("test-input-1.txt");
    println!("Initial state:");
    map.print();
    println!("");
    println!("Part 2 minimum path value: {}", 0);
}

fn find_paths(map: &Map) {
    let mut paths: Vec<Vec<(usize, Facing)>> = Vec::new();
    let index  = map.find_first('S').unwrap();
    let facing = Facing::East;

    let mut path: Vec<(usize, Facing)> = Vec::new();
    path.push((index, facing));
    paths.push(path);

    let mut i = 0;      // path # we are looking at
    loop {
        let (index, facing) = paths[i].last().unwrap();        
        
        // if we have reached the exit, then we are done with this path!
        if map.chars[*index] == 'E' {
            println!("Found exit! {:?}", paths[i]);
            i += 1;
            if i == paths.len() {
                break;
            } else {
                continue;
            }
        }

        // otherwise, try and get next steps
        let next_steps = find_allowed_steps(&map, *index, *facing);
        // if we have no more next steps, then we are done with this path
        if next_steps.len() == 0 {
            i += 1;
            paths[i].clear();
        } else if next_steps.len() == 1 {
            paths[i].push(next_steps[0]);
        } else {
            for k in 1..next_steps.len() {
                let mut new_path = paths[i].clone();
                new_path.push(next_steps[k]);
                paths.push(new_path);
            }
            // finally tack on the first next_step to our current path
            paths[i].push(next_steps[0]);
        }
        if i == paths.len() {
            break;
        }
    }
    println!("\nTotal paths tried: {}", i);

}

fn find_allowed_steps(map: &Map, index: usize, facing: Facing) -> Vec<(usize, Facing)> {
    let mut result: Vec<(usize, Facing)> = Vec::new();

    if map.up(index, '#') != '#' && facing != Facing::South {
        result.push((map.index_up(index).unwrap(), Facing::North));
    }
    if map.right(index, '#') != '#' && facing != Facing::West {
        result.push((map.index_right(index).unwrap(), Facing::East));
    }
    if map.down(index, '#') != '#' && facing != Facing::North {
        result.push((map.index_down(index).unwrap(), Facing::South));
    }
    if map.left(index, '#') != '#' && facing != Facing::East {
        result.push((map.index_left(index).unwrap(), Facing::West));
    }
    result
}