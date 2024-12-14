use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new() -> Self {
        Point{ x: 0, y: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
struct Robot {
    pos: Point,
    vel: Point
}

impl Robot {
    fn new(pos: Point, vel: Point) -> Self {
        Robot{ pos, vel }
    }
}

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
    let mut robots = load_robots("input.txt");
    let map = map_robots(&robots);
    move_robots(&mut robots, 100);
    let map = map_robots(&robots);
    quadrant_count(&robots, &map);
}

fn load_robots(path: &str) -> Vec<Robot> {
    let mut results = Vec::new();
    let file = File::open(path).unwrap();
    let mut buf = BufReader::new(file);
    let mut s = String::new();

    loop {
        let count = buf.read_line(&mut s).unwrap();
        if count > 0 {
            // format of line is "p=x,y v=x,y"
            let v: Vec<&str> = s.trim().split(' ').collect();
            // v[0] is our identiier now: Button A, Button B, Prize, or empty (end of record)
            let posxy: Vec<&str> = v[0].split('=').collect();    // parse p=x,y argument
            let pos = parse_x_comma_y(posxy[1]);         
            let velxy: Vec<&str> = v[1].split('=').collect();    // parse v=x,y argument
            let vel = parse_x_comma_y(velxy[1]);
            let robot = Robot::new(pos, vel);
            results.push(robot);
        }
        else {
            break;    
        }
        s.clear();
    }

    // return the vector of loaded Games
    results
}

fn parse_x_comma_y(x_comma_y: &str) -> Point {
    let xy: Vec<&str> = x_comma_y.split(',').collect();    // x,y argument
    let mut point = Point::new();
    point.x = xy[0].parse().unwrap();
    point.y = xy[1].parse().unwrap();
    point
}

fn get_space(robots: &Vec<Robot>) -> Point {
    let mut space = Point::new();
    if robots.len() <= 12 {
        space.x = 10;
        space.y = 6;
    } else {
        space.x = 100;
        space.y = 102;
    }
    space
}

fn move_robots(robots: &mut Vec<Robot>, num: i64) {
    let space = get_space(robots);

    for j in 0..robots.len() {
        let dx = num * robots[j].vel.x % (space.x + 1);
        let dy = num * robots[j].vel.y % (space.y + 1);
        robots[j].pos.x = robots[j].pos.x + dx;
        robots[j].pos.y = robots[j].pos.y + dy;
        if robots[j].pos.x < 0 {
            robots[j].pos.x = robots[j].pos.x + space.x + 1; 
        }
        if robots[j].pos.y < 0 {
            robots[j].pos.y = robots[j].pos.y + space.y + 1; 
        }
        if robots[j].pos.x > space.x {
            robots[j].pos.x = robots[j].pos.x - space.x - 1; 
        }
        if robots[j].pos.y > space.y {
            robots[j].pos.y = robots[j].pos.y - space.y - 1; 
        }
    }
}

fn map_robots(robots: &Vec<Robot>) -> Map {
    let space = get_space(robots);
    let mut map = Map::from_cols_rows(space.x as usize + 1, space.y as usize + 1);

    for robot in robots {
        let index = map.index_from_col_row(robot.pos.x as usize, robot.pos.y as usize);
        // if there is no robot there yet
        if map.chars[index] == '.' {
            map.chars[index] = '1';
        } else {
            let n: u32 = map.chars[index].to_digit(10).unwrap();
            map.chars[index] = char::from_digit(n + 1, 10).unwrap();
        }
    }
    map.print();
    println!("");
    map
}

fn quadrant_count(robots: &Vec<Robot>, map: &Map) {
    let space = get_space(robots);
    let mut qc = (0, 0, 0, 0);
    let mut ignored = 0;
    for (i, c) in map.chars.iter().enumerate() {
        // if this is a blank map square, then carry on
        if *c == '.' { continue; }
        // otherwise, we need to get the robot count
        let n: u32 = map.chars[i].to_digit(10).unwrap();
        // and put it in the correct quadrant count
        let (x, y) = map.col_row_from_index(i);
        // 1st quadrant?
        if x < space.x as usize / 2 && y < space.y as usize / 2 {
            qc.0 += n;
        } else if x > space.x as usize / 2 && y < space.y as usize / 2 {
            qc.1 += n;
        } else if x < space.x as usize / 2 && y > space.y as usize / 2 {
            qc.2 += n;
        } else if x > space.x as usize / 2 && y > space.y as usize / 2 {
            qc.3 += n;
        } else {
            ignored += n;
        }
    }
    println!("Quadrant count: {:?} - {} ignored", qc, ignored);
    println!("Safety factor: {}", qc.0 * qc.1 * qc.2 * qc.3);
    println!("");
}