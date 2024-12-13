use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new() -> Self {
        Point{ x: 0, y: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
struct Game {
    a: Point,
    b: Point,
    prize: Point
}

impl Game {
    fn new(a: Point, b: Point, prize: Point ) -> Game {
        Game { a, b, prize }
    }
}

fn main() {
    let mut games = load_games("input.txt");
    let mut total_cost = 0;
    for game in &games {
        let cost = find_cost(&game);
        println!("{:?} cost is {}", game, cost);
        total_cost += cost;
    }
    println!("Part 1 total cost: {total_cost}");
    
    // for part 2, we need to adjust all game prize x,y coords to be += 10000000000000
    let mut total_cost = 0;
    for i in 0..games.len() {
        games[i].prize.x += 10000000000000;
        games[i].prize.y += 10000000000000;
        let cost = find_cost(&games[i]);
        println!("{:?} cost is {}", games[i], cost);
        total_cost += cost;
    }
    println!("Part 2 total cost: {total_cost}");
}

fn load_games(path: &str) -> Vec<Game> {
    let mut results = Vec::new();
    let file = File::open(path).unwrap();
    let mut buf = BufReader::new(file);
    let mut s = String::new();
    let mut a = Point::new();
    let mut b = Point::new();
    let mut prize = Point::new();

    loop {
        let count = buf.read_line(&mut s).unwrap();
        if count > 0 {
            // format of line is "COMMAND: args"
            let v: Vec<&str> = s.split(':').collect();
            // v[0] is our identiier now: Button A, Button B, Prize, or empty (end of record)
            if v[0] == "Button A" {
                let args = String::from(v[1]);    // parse button A line
                let args: Vec<&str> = args.split(",").map(|arg| arg.trim()).collect();
                let x: Vec<&str> = args[0].split("+").collect();
                let x = String::from(x[1]);
                a.x = x.parse().unwrap();
                let y: Vec<&str> = args[1].split("+").collect();
                let y = String::from(y[1]);
                a.y = y.parse().unwrap();
                //println!("Command: {}, args: {:?}, x: {}, y: {}", v[0], args, a.x, a.y);
            } else if v[0] == "Button B" {
                let args = String::from(v[1]);    // parse button A line
                let args: Vec<&str> = args.split(",").map(|arg| arg.trim()).collect();
                let x: Vec<&str> = args[0].split("+").collect();
                let x = String::from(x[1]);
                b.x = x.parse().unwrap();
                let y: Vec<&str> = args[1].split("+").collect();
                let y = String::from(y[1]);
                b.y = y.parse().unwrap();
                //println!("Command: {}, args: {:?}, x: {}, y: {}", v[0], args, x, y);
            } else if v[0] == "Prize" {
                let args = String::from(v[1]);    // parse button A line
                let args: Vec<&str> = args.split(",").map(|arg| arg.trim()).collect();
                let x: Vec<&str> = args[0].split("=").collect();
                let x = String::from(x[1]);
                prize.x = x.parse().unwrap();
                let y: Vec<&str> = args[1].split("=").collect();
                let y = String::from(y[1]);
                prize.y = y.parse().unwrap();
                //println!("Command: {}, args: {:?}, x: {}, y: {}", v[0], args, x, y);      
            } else {
                // time to add a new game!
                let mut game = Game::new(a, b, prize);
                //println!("{:?}", game);
                results.push(game);
            }
        }
        else {
            break;    
        }
        s.clear();
    }


    // return the vector of loaded Games
    results
}

fn find_cost(game: &Game) -> usize {
    let mut result = 0;
    // a button loop
    let mut i = 1;
    loop {
        let a_pos = Point{ x: game.a.x * i, y: game.a.y * i };
        if a_pos.x > game.prize.x || a_pos.y > game.prize.y || (result > 0 && i * 3 > result) { break; } 
        //we can now compute the # of b button presses needed
        let dx = game.prize.x - a_pos.x;
        let dy = game.prize.y - a_pos.y;
        let bx =  dx / game.b.x;
        let mx = dx % game.b.x;
        let by = dy / game.b.y;
        let my = dy % game.b.y;
        // do we have an exact target point?
        if bx == by && mx == 0 && my == 0 {
            let j = bx;
            let cost = i * 3 + j;
            if result == 0 {
                println!("(a,b) = ({},{}) cost is {}", i, j, cost);
                result = cost;
            } else {
                result = cmp::min(result, cost);
            }
        }
        i += 1;
    }
    result
}