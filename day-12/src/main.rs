use std::collections::BTreeMap;
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
    let farm = Map::from_file("input.txt");
    println!("");
    println!(
        "Farm is {} cols by {} rows.",
        farm.cols, farm.rows
    );
    farm.print();
    println!("");
    // create our plot map
    let mut plots: BTreeMap<char, Vec<Vec<usize>>> = BTreeMap::new();
    let mut total_fence_cost = 0;
    let mut part2_cost = 0;
    // build our list of plots for each crop type, visiting each cell of our farm
    for (i,c) in farm.chars.iter().enumerate() {
        if !plots.contains_key(c) {
            // then we have a new plot for sure! Add it to our plot map
            // create a plot list for this crop
            //println!("First plot found for crop {}.", c);
            let mut char_plots: Vec<Vec<usize>> = Vec::new();
            let new_plot = map_plot(i, *c, &farm);
            //if *c == 'E' { println!("{:?}", new_plot); }
            let area = new_plot.len();
            let perimeter = plot_perimeter(&new_plot, &farm);
            let sides = count_corners(&new_plot, &farm);
            let fence_cost = area * perimeter;
            total_fence_cost += fence_cost;
            part2_cost += sides * area;
            println!("Area * sides = fence cost for plot '{}': {} * {} = {}", *c, area, sides, sides * area);
            char_plots.push(new_plot);
            plots.insert(*c, char_plots);
        } else {
            // ok, so we already have a plot recorded for this character
            // is this square in one of the plots already?
            let mut in_plot = false;
            let plot_list  = plots.get_mut(c).unwrap();
            for j in 0..plot_list.len() {
                if plot_list[j].contains(&i) {
                    in_plot = true;
                    break;
                }
            }
            // if this square is not in a plot already, then we must have another plot!
            if !in_plot {
                //println!("Another plot found for crop {}.", c);
                let new_plot = map_plot(i, *c, &farm);
                let area = new_plot.len();
                let perimeter = plot_perimeter(&new_plot, &farm);
                let sides = count_corners(&new_plot, &farm);
                let fence_cost = area * perimeter;
                total_fence_cost += fence_cost;
                part2_cost += sides * area;
                //println!("Area * perimiter = fence cost for plot '{}': {} * {} = {}", *c, area, perimeter, fence_cost);
                plot_list.push(new_plot);
            }
        }
    }
    println!("\nPart 1 total fence cost: {}", total_fence_cost);
    println!("\nPart 2 total fence cost: {}", part2_cost);
}

fn map_plot(i: usize, c: char, farm: &Map) -> Vec<usize> {
    let mut results = Vec::new();
    walk_plot(i, c, farm, &mut results);
    results.sort();
    results
}

fn walk_plot(i: usize, c: char, farm: &Map, plot: &mut Vec<usize>) {
    // do we have a matching crop?
    if farm.chars[i] == c {
        // yes, we do! add it to our plot list if not already in it
        if !plot.contains(&i) {
            plot.push(i);
            // check the crops up, right, and down from this position
            if farm.up(i, '.') == c {
                walk_plot( farm.index_up(i).unwrap(), c, farm, plot);
            }
            if farm.right(i, '.') == c {
                walk_plot( farm.index_right(i).unwrap(), c, farm, plot);
            }
            if farm.down(i, '.') == c {
                walk_plot( farm.index_down(i).unwrap(), c, farm, plot);
            }
            if farm.left(i, '.') == c {
                walk_plot( farm.index_left(i).unwrap(), c, farm, plot);
            }
        }
    }
}

fn plot_perimeter(plot: &Vec<usize>, farm: &Map) -> usize {
    let c = farm.chars[plot[0]];
    let mut perimeter = 0;
    for index in plot {
        let mut p = 4;  // starting perimeter for this index is 4, less one for each adjacent square
        if farm.up(*index, '.') == c {
            p -= 1;
        }
        if farm.right(*index, '.') == c {
            p -= 1;
        }
        if farm.down(*index, '.') == c {
            p -= 1;
        }
        if farm.left(*index, '.') == c {
            p -= 1;
        }
        perimeter += p;
    }
    perimeter
}

fn count_corners(plot: &Vec<usize>, farm: &Map) -> usize {
    let c = farm.chars[plot[0]];
    let mut corners = 0;
    for index in plot {
        let mut i = 0;  // starting perimeter for this index is 4, less one for each adjacent square
        let up = farm.up(*index, '.') == c;
        let up_right = farm.up_right(*index, '.') == c;
        let right = farm.right(*index, '.') == c;
        let down_right = farm.down_right(*index, '.') == c;
        let down = farm.down(*index, '.') == c;
        let down_left = farm.down_left(*index, '.') == c;
        let left = farm.left(*index, '.') == c;
        let up_left = farm.up_left(*index, '.') == c;

        // count our corners (max of 4)
        if !up && !right { i += 1; }
        if !right && !down { i += 1; }
        if !down && !left { i += 1; }
        if !left && !up { i += 1; }
        if up && right && !up_right { i += 1; }
        if down && right && !down_right { i += 1; }
        if down && left && !down_left { i += 1; }
        if up && left && !up_left { i += 1; }
        corners += i;
    }
    corners
}