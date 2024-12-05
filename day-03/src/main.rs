fn main() {
    let program = String::from(include_str!("../input.txt"));
    // go character by character and try to parse out instruction
    let mut i = 0_usize;
    let mut sum = 0_i32;
    let mut enable_mul = true;
    while i < program.len() {
        i += parse(i, &program, &mut sum, &mut enable_mul);
    }
    println!("Part 1 Sum: {}", sum);
}

// assumption - all instructions will look as follows:
// instr_name(args).
fn parse(i: usize, program: &String, sum: &mut i32, enable_mul: &mut bool) -> usize {
    let instructions = ["mul(", "do()", "don't()"];
    let mut count = 1;
    // does the current index point to a valid instruction?
    for n in 0..instructions.len() {
        if i + instructions[n].len() < program.len() {
            let token = &program[i..i + instructions[n].len()];
            if token == instructions[n] {
                count = instructions[n].len();
                if token == "mul(" {
                    // parse arg1 as i32
                    let mut args = vec![0, 0];
                    if let Some((arg1, len)) = get_arg_i(&program[i + count..]) {
                        args[0] = arg1;
                        count += len;
                    } else {
                        break;
                    }
                    // need a comma next
                    if &program[i + count..i + count + 1] != "," {
                        break;
                    }
                    count += 1;
                    // parse arg2 as i32
                    if let Some((arg2, len)) = get_arg_i(&program[i + count..]) {
                        args[1] = arg2;
                        count += len;
                    } else {
                        break;
                    }
                    // need a closing paren
                    if &program[i + count..i + count + 1] != ")" {
                        break;
                    }
                    count += 1;
                    if *enable_mul {
                        println!("mul({},{})", args[0], args[1]);
                        *sum += args[0] * args[1];    
                    }
                } else if token == "do()" {
                    println!("do()");
                    *enable_mul = true;
                } else if token == "don't()" {
                    println!("don't()");
                    *enable_mul = false;                
                }
                break;
            }
        }
    }
    count
}

fn get_arg_i(program: &str) -> Option<(i32, usize)> {
    let mut n = 0_usize;
    for c in program.chars() {
        if c.is_digit(10) {
            n += 1;
        } else {
            break;
        }
    }
    if n > 0 {
        Some((program[..n].parse::<i32>().unwrap(), n))
    } else {
        None
    }
}
