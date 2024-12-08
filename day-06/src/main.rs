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
    let (mut guard_col, mut guard_row) = col_row_from_index(guard, cols);
    let guard_origin = guard;
    // save the guard's path for later analysis
    let mut path: Vec<usize> = Vec::new();
    path.push(guard);
    // ok, we now loop until the guard leaves the map
    loop {
        if !move_guard(&mut guard_col, &mut guard_row, cols, rows, &mut map) {
            print_map(&map, cols);
            break;
        }
        path.push(index_from_col_row(guard_col, guard_row, cols));
    }
    let mut unique = path.clone();
    unique.sort();
    unique.dedup();
    println!(
        "Locations visited: {} total, {} unique",
        path.len(),
        unique.len()
    );

    // now, part 2! Remove guard's origin from list of unique locations,
    // since we can not place an obstacle there. 
    unique.retain(|x| *x != guard_origin);
    let mut blockers = 0_u32;
    // iterate over all of the unique locations in the guard's path
    for obs in unique {
        // transform all 'X's to '.'s
        for m in 0..map.len() {
            if map[m] == 'X' || map[m] == '^' || map[m] == '>' || map[m] == 'v' || map[m] == '<' || map[m] == '@' {
                map[m] = '.';
            }
        }
        // place the guard back at the origin and set the obstacle
        map[guard_origin] = '^';
        map[obs] = 'O';
        // test if we have a loop here ...
        if is_looping(&mut map, cols, rows) {
            blockers += 1;
            println!("Loop #{}", blockers);
            //break;
        }
        // reset obstacle to clear for next loop
        map[obs] = '.';
    }
    println!("Obstacle locations: {}", blockers);
}

fn is_looping(map: &mut Vec<char>, cols: usize, rows: usize) -> bool {
    // get the guard's starting position
    let mut current = map.iter().position(|&c| c == '^').unwrap();
    let mut is_loop = false;
    let mut path: Vec<usize> = Vec::new();

    //println!("Testing:");
    //print_map(&map, cols);

    // ok, we now loop until the guard leaves the map OR we detect a loop
    loop {
        let (mut col, mut row) = col_row_from_index(current, cols);
        if !move_guard(&mut col, &mut row, cols, rows, map) {
            break;
        }
        let next = index_from_col_row(col, row, cols);
        // now check our stored path to see if we are looping
        let mut match_current = false;
        for p in &path {
            if *p == current {
                match_current = true;
            } else if *p == next && match_current {
                is_loop = true;
                break;
            } else {
                match_current = false;
            }
        }
        path.push(next);
        if is_loop {
            map[current] = '@';
            print_map(&map, cols);
            break; 
        }
        current = next;
    }
    // if we make it this far, then no loop
    is_loop
}

fn col_row_from_index(index: usize, cols: usize) -> (usize, usize) {
 (index % cols, index / cols)
}

fn index_from_col_row(col: usize, row: usize, cols: usize) -> usize {
    row * cols + col
}

fn is_blocked(col: usize, row: usize, cols: usize, map: &Vec<char>) ->bool {
    let c = map[row * cols + col];
    (c == '#') || (c == 'O')
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
    map[*guard_row * cols + *guard_col] = guard;
    loop {
        if guard == '^' {
            if *guard_row == 0 {
                // we can't move any farther up!
                return false;
            }
            // make sure the new location sn't blocked
            if is_blocked(*guard_col, *guard_row - 1, cols, map) {
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
            if is_blocked(*guard_col + 1, *guard_row, cols, map) {
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
            if is_blocked(*guard_col, *guard_row + 1, cols, map) {
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
            if is_blocked(*guard_col - 1, *guard_row, cols, map) {
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