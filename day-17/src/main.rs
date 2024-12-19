
#[derive(Debug)]
struct Computer {
    a: usize,          // A register
    b: usize,          // B register
    c: usize,          // C register
    ip: usize,         // instruction pointer
    ram: Vec<u8>,      // contents of RAM
    stdout: Vec<u8>,   // standard output
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, program: Vec<u8>) -> Self {
        Self {a: a, b: b, c: c, ip: 0, ram: program, stdout: Vec::new(), }
    }

    fn exec(&mut self) {
        // implement our fetch and execute loop
        // for now, always start at IP = 0 and clear stdout
        self.ip = 0;
        self.stdout.clear();
        // loop until we run past memory, which causes the machine to HALT
        while self.ip < self.ram.len() {
            let opcode = self.fetch_opcode();
            let operand = self.fetch_operand(opcode);
            // now process the instruction
            self.process_instruction(opcode, operand);
        }
    }

    fn fetch_opcode(&mut self) -> u8 {
        self.ip += 1;
        self.ram[self.ip - 1]
    }

    fn fetch_operand(&mut self, _opcode: u8) -> u8 {
        // since all opcodes always have 1 operand, it is sort of
        // silly to pass in the opcode. However, if part 2 expands
        // instructions to allow for a variable number of opcodes,
        // then this routine can be altered to support that. 
        self.ip += 1;
        self.ram[self.ip - 1]
    }

    fn fetch_combo_operand_value(&self, operand: u8) -> usize {
        match operand {
        0..=3 => operand as usize,
        4 => self.a,
        5 => self.b,
        6 => self.c,
        _ => panic!("Invalid combo operand value: {}", operand)
        }
    }

    fn process_instruction(&mut self, opcode: u8, operand: u8) {
        match opcode {
            0 => self.a = self.a >> self.fetch_combo_operand_value(operand) as u32, // adv - divide A register by 2^combo_op => A
            1 => self.b = self.b ^ operand as usize,                                // bxl - bitwise XOR of B register by op => B
            2 => self.b = self.fetch_combo_operand_value(operand) % 8,              // bst - combo_op % 8 => B
            3 => if self.a != 0 { self.ip = operand as usize; }                     // jnz - jump to address = operand if A is != 0
            4 => self.b = self.b ^ self.c,                                          // bxs - bitwise XOR B & C => B
            5 => {
                self.stdout.push((self.fetch_combo_operand_value(operand) % 8) as u8);
            }
            ,   // out - print out combo_op mod 8
            6 => self.b = self.a >> self.fetch_combo_operand_value(operand) as u32, // bdv - shr B combo_op => B
            7 => self.c = self.a >> self.fetch_combo_operand_value(operand) as u32, // cdv - shr C combo_op => C
            _ => println!("INVALID OPCODE: {}\n", opcode),
        }
    }

    fn opcode_as_string(opcode: u8) -> String {
        match opcode {
            0 | 6 | 7=> return "shr".to_string(),
            1 | 4 => return "xor".to_string(),
            2 => return "mod".to_string(),
            3 => return "jnz".to_string(),
            5 => return "out".to_string(),
            _ => return "???".to_string(),
        }
    }

    fn operand_as_string(opcode: u8, operand: u8) -> String {
        match opcode {
            0 => return format!("A, {}\t=> A", Self::combo_operand_as_string(operand)),
            1 => return format!("B, #{}\t=> B", operand),
            2 => return format!("{}, 8\t=> B", Self::combo_operand_as_string(operand)),
            3 => return format!("${:04}", operand),
            4 => return "B, C\t=> B".to_string(),
            5 => return Self::combo_operand_as_string(operand),
            6 => return format!("A, {}\t=> B", Self::combo_operand_as_string(operand)),
            7 => return format!("A, {}\t=> C", Self::combo_operand_as_string(operand)),
            _ => return "???".to_string(),
        }
    }

    fn combo_operand_as_string(operand: u8) -> String {
        match operand {
            0..=3 => return format!("#{}", operand),
            4 => return "A".to_string(),
            5 => return "B".to_string(),
            6 => return "C".to_string(),
            _ => return "?".to_string(),
        }
    }

    fn disassemble(&self) {
        for i in 0..self.ram.len() / 2 {
            // get our opcode and operand
            let opcode = self.ram[i * 2];
            let operand = self.ram[i * 2 + 1];
            println!("{:04}: {:02} {:02}\t{}\t{}", i * 2, opcode, operand, Self::opcode_as_string(opcode), Self::operand_as_string(opcode, operand));
        }
    }

}

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("\nPart 1:");
    let code = vec![2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0];
    let mut hal = Computer::new(46337277, 0, 0, code);
    hal.exec();
    println!("{:?}", hal.stdout);
}

fn part2() {
    println!("\nPart 2:");
    let code = vec![2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0];
    let mut hal = Computer::new(0, 0, 0, code);
    hal.a = 202991746427434;
    hal.disassemble();
    hal.exec();
    println!("   code: {:?}", hal.ram);
    println!(" stdout: {:?}", hal.stdout);
}

fn check_partial_solution(check: &Vec<u8>) {
    let code = vec![2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0];
    let mut hal = Computer::new(0, 0, 0, code);
    for i in 0..check.len() {
        // build up our test a value with the 3 bit values we have
        hal.a = hal.a + ((check[i] as usize) << ((15-i) * 3));
    }
    print!("A: {} ", hal.a);
    hal.exec();
    println!("{:?} => {:?}", check, hal.stdout);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let code = vec![2,6];
        let mut hal = Computer::new(0, 0, 9, code);
        hal.exec();
        assert_eq!(hal.b, 1);
    }

    #[test]
    fn part1_test2() {
        let code = vec![5,0,5,1,5,4];
        let mut hal = Computer::new(10, 0, 0, code);
        hal.exec();
        let s = format!("{:?}", hal.stdout);
        assert_eq!(s, "[0, 1, 2]");
    }

    #[test]
    fn part1_test3() {
        let code = vec![0,1,5,4,3,0];
        let mut hal = Computer::new(2024, 0, 0, code);
        let s = format!("{:?}", hal.stdout);
        assert_eq!(s, "[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]");
        assert_eq!(hal.a, 0);
    }

    #[test]
    fn part1_test4() {
        let code = vec![1,7];
        let mut hal = Computer::new(0, 29, 0, code);
        hal.exec();
        assert_eq!(hal.b, 26);
    }

    #[test]
    fn part1_test5() {
        let code = vec![4,0];
        let mut hal = Computer::new(0, 2024, 43690, code);
        hal.exec();
        assert_eq!(hal.b, 44354);
    }

    #[test]
    fn part1_test6() {
        let code = vec![0,1,5,4,3,0];
        let mut hal = Computer::new(729, 0, 0, code);
        hal.exec();
        let s = format!("{:?}", hal.stdout);
        assert_eq!(s, "[4, 6, 3, 5, 6, 3, 5, 2, 1, 0]");
    }

    #[test]
    fn part2_test1() {
        let code = vec![0,3,5,4,3,0];
        let mut hal = Computer::new(117440, 0, 0, code);
        hal.exec();
        let s = format!("{:?}", hal.stdout);
        assert_eq!(s, "[0, 3, 5, 4, 3, 0]");
    }

}