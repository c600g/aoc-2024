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
}

fn main() {
    let antennas = Map::from_file("input.txt");
    println!("");
    println!(
        "Antenna map is {} cols by {} rows.",
        antennas.cols, antennas.rows
    );
    antennas.print();
    println!("");

    let antinodes = create_antinodes(&antennas, false);
    println!("Part 1 Antinodes map:");
    antinodes.print();

    println!(
        "Total # of part 1 antinodes: {}",
        antinodes.chars.iter().filter(|&a| *a != '.').count()
    );

    println!("");
    let antinodes = create_antinodes(&antennas, true);
    println!("Part 2 Resonant Antinodes map:");
    antinodes.print();

    println!(
        "Total # of part 2 antinodes: {}",
        antinodes.chars.iter().filter(|&a| *a != '.').count()
    );
}

fn create_antinodes(antennas: &Map, resonant: bool) -> Map {
    // analyze our antenna map to generate the antinodes map
    let mut map = Map::from_cols_rows(antennas.cols, antennas.rows);

    for (i, pos) in antennas.chars.iter().enumerate() {
        // if this is an open location ('.') then continue nwards
        if *pos == '.' {
            continue;
        }
        // ok, we have an antenna - time to find all other similar antennas
        // and map out their antinode locations
        for j in i + 1..antennas.chars.len() {
            // if we have a matching antenna
            if antennas.chars[j] == *pos {
                // then we map out the antinodes here
                if !resonant {
                    generate_antinodes(i, j, &mut map);
                } else {
                    generate_resonant_antinodes(i, j, &mut map);
                }
            }
        }
    }
    map
}

fn generate_antinodes(i: usize, j: usize, antinodes: &mut Map) {
    let (x1, y1) = antinodes.col_row_from_index(i);
    let (x2, y2) = antinodes.col_row_from_index(j);
    // create vector dx, dy
    let (dx, dy) = (x2 as i64 - x1 as i64, y2 as i64 - y1 as i64);
    // create antinodes: x2,y2 + dx,dy and x1,y1 - dx,dy
    let an1 = (x2 as i64 + dx, y2 as i64 + dy);
    let an2 = (x1 as i64 - dx, y1 as i64 - dy);
    // if antinode 1 is on the map, then mark it!
    if an1.0 >= 0 && an1.0 < antinodes.cols as i64 && an1.1 >= 0 && an1.1 < antinodes.rows as i64 {
        let index = antinodes.index_from_col_row(an1.0 as usize, an1.1 as usize);
        antinodes.chars[index] = '#';
    }
    // if antinode 2 is on the map, then mark it!
    if an2.0 >= 0 && an2.0 < antinodes.cols as i64 && an2.1 >= 0 && an2.1 < antinodes.rows as i64 {
        let index = antinodes.index_from_col_row(an2.0 as usize, an2.1 as usize);
        antinodes.chars[index] = '#';
    }
}

fn generate_resonant_antinodes(i: usize, j: usize, antinodes: &mut Map) {
    let (x1, y1) = antinodes.col_row_from_index(i);
    let (x2, y2) = antinodes.col_row_from_index(j);
    // both antennas are antinodes
    antinodes.chars[i] = '#';
    antinodes.chars[j] = '#';

    // create vector dx, dy
    let (dx, dy) = (x2 as i64 - x1 as i64, y2 as i64 - y1 as i64);

    // generate resonant nodes for x1, y1
    let mut an1 = (x2 as i64, y2 as i64);
    loop {
        an1 = (an1.0 + dx, an1.1 + dy);
        let valid = an1.0 >= 0
            && an1.0 < antinodes.cols as i64
            && an1.1 >= 0
            && an1.1 < antinodes.rows as i64;
        if !valid {
            break;
        }
        let index = antinodes.index_from_col_row(an1.0 as usize, an1.1 as usize);
        antinodes.chars[index] = '#';
    }
    // generate resonant nodes for x1, y1
    let mut an2 = (x1 as i64, y1 as i64);
    loop {
        an2 = (an2.0 - dx, an2.1 - dy);
        let valid = an2.0 >= 0
            && an2.0 < antinodes.cols as i64
            && an2.1 >= 0
            && an2.1 < antinodes.rows as i64;
        if !valid {
            break;
        }
        let index = antinodes.index_from_col_row(an2.0 as usize, an2.1 as usize);
        antinodes.chars[index] = '#';
    }
}
