use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf = BufReader::new(file);
    let mut s = String::new();
    let mut map: Vec<char> = Vec::new();
    let mut cols = 0_usize;
    let mut rows = 0_usize;

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
    // find the guard's initial position
    let guard = map.iter().position(|&c| c == '^').unwrap();
    let mut guard_row = (guard + 1) / cols;
    let mut guard_col = (guard + 1) % cols - 1;
    // ok, we now loop until the guard leaves the map
    loop {
        if !move_guard(&mut guard_col, &mut guard_row, cols, rows, &mut map) {
            print_map(&map, cols);
            break;
        }
    }
    println!(
        "Locations visited: {}",
        map.iter().filter(|&n| *n == 'X').count()
    );
}

fn move_guard(
    guard_col: &mut usize,
    guard_row: &mut usize,
    cols: usize,
    rows: usize,
    map: &mut Vec<char>,
) -> bool {
    // get the guard character to determine direction of travel
    let mut guard: char = map[*guard_row * cols + *guard_col];
    // mark current location as visited
    map[*guard_row * cols + *guard_col] = 'X';
    loop {
        if guard == '^' {
            if *guard_row == 0 {
                // we can't move any farther up!
                return false;
            }
            // make sure the new location sn't blocked
            let is_blocked = map[(*guard_row - 1) * cols + *guard_col] == '#';
            if is_blocked {
                guard = '>';
            } else {
                *guard_row -= 1;
                map[*guard_row * cols + *guard_col] = guard;
                return true;
            }
        }

        if guard == '>' {
            if *guard_col == cols - 1 {
                // we can't move any farther right!
                return false;
            }
            // make sure the new location sn't blocked
            let is_blocked = map[*guard_row * cols + *guard_col + 1] == '#';
            if is_blocked {
                guard = 'v';
            } else {
                *guard_col += 1;
                map[*guard_row * cols + *guard_col] = guard;
                return true;
            }
        }

        if guard == 'v' {
            if *guard_row == rows - 1 {
                // we can't move any farther down!
                return false;
            }
            // make sure the new location sn't blocked
            let is_blocked = map[(*guard_row + 1) * cols + *guard_col] == '#';
            if is_blocked {
                guard = '<';
            } else {
                *guard_row += 1;
                map[*guard_row * cols + *guard_col] = guard;
                return true;
            }
        }

        if guard == '<' {
            if *guard_col == 0 {
                // we can't move any farther right!
                return false;
            }
            // make sure the new location sn't blocked
            let is_blocked = map[*guard_row * cols + *guard_col - 1] == '#';
            if is_blocked {
                guard = '^';
            } else {
                *guard_col -= 1;
                map[*guard_row * cols + *guard_col] = guard;
                return true;
            }
        }
    }
}

fn print_map(map: &Vec<char>, cols: usize) {
    for (i, c) in map.iter().enumerate() {
        if i % cols == 0 {
            print!("\n");
        }
        print!("{}", c);
    }
    println!("");
}