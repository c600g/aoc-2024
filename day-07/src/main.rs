use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
enum Ops {
    Add,
    Mul,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf = BufReader::new(file);
    let mut s = String::new();
    let mut total = 0;
    let mut arg_ops: Vec<Vec<Vec<Ops>>> = Vec::new();
    arg_ops.push(vec![vec![]]);
    loop {
        let count = buf.read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if count > 0 && s.len() > 0 {
            let colon = s.find(":").unwrap();
            let sum: usize = (&s[..colon].trim())
                .to_string()
                .parse()
                .expect("Can not parse.");
            let args = (&s[colon + 1..].trim()).to_string();
            let args: Vec<usize> = args
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            println!("sum:{} args: {:?}", sum, args);
            // get the ops vector for this arg count
            let operations = get_arg_ops(args.len(), &mut arg_ops);
            for ops in operations {
                if test_ops(sum, &args, ops) {
                    total += sum;
                    break;
                }
            }
        } else {
            break;
        }
        s.clear();
    }
    println!("Total: {}", total);
}

fn get_arg_ops(argc: usize, arg_ops: &mut Vec<Vec<Vec<Ops>>>) -> &Vec<Vec<Ops>> {
    let mut argc = argc;
    if argc == 0 {
        argc = 1;
    }
    if argc - 1 < arg_ops.len() {
        return &arg_ops[argc - 1];
    } else {
        println!("Generating ops vectors for argc={}", argc);
        let mut next: Vec<Vec<Ops>> = Vec::new();
        if argc == 1 {
            // do nada
        } else if argc == 2 {
            next.push(vec![Ops::Add]);
            next.push(vec![Ops::Mul]);
        } else {
            let prev = get_arg_ops(argc - 1, arg_ops);
            for op in vec![Ops::Add, Ops::Mul] {
                for oplist in prev {
                    let mut new = oplist.to_vec();
                    new.push(op.clone());
                    next.push(new);
                }
            }
        }
        arg_ops.push(next);
        println!("Arg ops is: {:?}", arg_ops[argc - 1]);
        return &arg_ops[argc - 1];
    }
}

fn test_ops(sum: usize, args: &Vec<usize>, ops: &Vec<Ops>) -> bool {
    let mut test = 0;

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            test = *arg;
        } else {
            // we have an arg # > 1, so we need to get the corresponding operator
            // from the list
            let op = ops[i - 1];
            test = eval(op, test, *arg);
        }
    }

    // return true if test == sum
    test == sum
}

fn eval(op: Ops, arg1: usize, arg2: usize) -> usize {
    match op {
        Ops::Add => arg1 + arg2,
        Ops::Mul => arg1 * arg2
    }
}
