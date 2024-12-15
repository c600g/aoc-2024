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
                return Some(i)
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


fn main() {
    part1();
    part2();
}

fn part1() {
    let (mut map, moves) = load_input("input.txt");
    let mut robot_location = map.find_first('@').unwrap();
    // println!("Initial state:");
    // map.print();
    // println!("");
    for dir in &moves {
        robot_location = move_object(&mut map, *dir, robot_location);
        // println!("{}", dir);
        // map.print();
        // println!("");
    }
    println!("Part 1 sum of GPS coords: {}", count_gps(&map));
}

fn part2() {
    // now, for part 2 wherein the map is doubled!
    let (mut map, moves) = load_input("test-input-3.txt");
    let mut new_map = double_map(&map);
    let mut robot_location = new_map.find_first('@').unwrap();
    println!("Initial state:");
    new_map.print();
    println!("");
    for dir in &moves {
        robot_location = move_object(&mut new_map, *dir, robot_location);
        println!("{}", dir);
        new_map.print();
        println!("");
    }
    println!("Part 2 sum of GPS coords: {}", count_gps(&new_map));
}

fn load_input(path: &str) -> (Map, Vec<char>) {
    let file = File::open(path).unwrap();
    let mut buf = BufReader::new(file);
    //let mut s = String::new();
    let mut map = Map::new();
    let mut moves: Vec<char> = Vec::new();
    let mut map_data = true;

    loop {
        let mut s = String::new();
        let count = buf.read_line(&mut s).unwrap();
        if count > 0 {
            if map_data {
                s = s.trim().to_string();
                if s.len() == 0 {
                    map_data = false;
                    continue;
                }
                map.cols = s.len();
                let mut string_chars: Vec<char> = s.chars().collect();
                map.chars.append(&mut string_chars);
                map.rows += 1;    
            } else {
                // append moves
                let mut string_chars: Vec<char> = s.chars().collect();
                moves.append(&mut string_chars);
            }
        } else {
            break;
        }
    }
    (map, moves)
}

fn move_object(map: &mut Map, dir: char, location: usize) -> usize {
    let mut new_location = location;
    if dir == '^' {
        // get the next location up
        new_location = map.index_up(location).unwrap();
    } else if dir == '>' {
        new_location = map.index_right(location).unwrap();
    } else if dir == 'v' {
        new_location = map.index_down(location).unwrap();
    } else if dir == '<' {
        new_location = map.index_left(location).unwrap();
    }
    // get the character in the new location
    let c = map.chars[new_location];
    // if the new location is open space, then move the object and return that location!
    if c == '.' {
        // go ahead and move what is in location to new_location
        map.chars[new_location] = map.chars[location];
        map.chars[location] = '.';
    } 
    // if the new location is a movable object
    else if c == 'O' {
        // then try and move that object in the same direction
        let moved_obj_loc = move_object(map, dir, new_location);
        // if it was not able to be moved, then we can't move this object too
        if moved_obj_loc == new_location {
            new_location = location;
        } else {
            map.chars[new_location] = map.chars[location];
            map.chars[location] = '.';    
        }
    }
    // if the new location is a movable object
    else if c == '[' || c == ']' {
        // we need special log for up/down moves since a box is now 2 characters wide!
        if dir == '^' || dir == 'v' {
            // for now, just disallow up/down moves by doing nothing
            new_location = location;
        } else {
            // then try and move that object in the same direction
            let mut moved_obj_loc = move_object(map, dir, new_location);
            // if it was not able to be moved, then we can't move this object too
            if moved_obj_loc == new_location {
                new_location = location;
            } else {
                map.chars[new_location] = map.chars[location];
                map.chars[location] = '.';    
            }
        }
    }
    // otherwise, it must be blocking terrain, so no movement!
    else {
        new_location = location;
    }
    new_location
}

fn count_gps(map: &Map) -> usize {
    let mut result = 0;
    for (i, c) in map.chars.iter().enumerate() {
        // if this is a box of goods
        if *c == 'O' || *c == '[' {
            // then get the gps coords
            let (x, y) = map.col_row_from_index(i);
            result += 100 * y + x;
        }
    }
    result
}

fn double_map(map: &Map) -> Map {
    let mut result = Map::from_cols_rows(map.cols * 2, map.rows);
    for (i, c) in map.chars.iter().enumerate() {
        match *c {
            '.' | '#' => { result.chars[i * 2] = *c; result.chars[i * 2 + 1] = *c;  },
            'O' => { result.chars[i * 2] = '['; result.chars[i * 2 + 1] = ']';  }
            '@' => { result.chars[i * 2] = '@'; result.chars[i * 2 + 1] = '.';  }
            _ => {},
            
        }
    }
    result
}