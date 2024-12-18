
#[derive(Debug)]
struct Computer {
    a: usize,       // A register
    b: usize,       // B register
    c: usize,       // C register
    ip: usize,      // instruction pointer
    ram: Vec<u8>,   // contents of RAM
    stdout: String, // standard output
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, program: Vec<u8>) -> Self {
        Self {a: a, b: b, c: c, ip: 0, ram: program, stdout: String::new() }
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
            0 => self.a = self.a / 2_usize.pow(self.fetch_combo_operand_value(operand) as u32), // adv - divide A register by 2^combo_op => A
            1 => self.b = self.b ^ operand as usize,                               // bxl - bitwise XOR of B register by op => B
            2 => self.b = self.fetch_combo_operand_value(operand) & 7,             // bst - combo_op & 7 => B
            3 => if self.a != 0 { self.ip = operand as usize; }                    // jnz - jump to address = operand if A is != 0
            4 => self.b = self.b ^ self.c,                                         // bxs - bitwise XOR B & C => B
            5 => {
                if self.stdout.len() > 0 {
                    self.stdout.push_str(",");
                }
                let s = format!("{}", self.fetch_combo_operand_value(operand) & 7);
                self.stdout.push_str(&s);
            }
            ,   // out - print out combo_op mod 8
            6 => self.b = self.a / 2_usize.pow(self.fetch_combo_operand_value(operand) as u32), // bdv - divide B register by 2^combo_op => B
            7 => self.c = self.a / 2_usize.pow(self.fetch_combo_operand_value(operand) as u32), // cdv - divide C register by 2^combo_op => C
            _ => println!("INVALID OPCODE: {}\n", opcode),
        }
    }

}

fn main() {
    let code = vec![2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0];
    let mut hal = Computer::new(46337277, 0, 0, code);
    println!("\nHal is running...");
    hal.exec();
    println!("stdout:\n{}\n\n", hal.stdout);
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
        assert_eq!(hal.stdout, "0,1,2");
    }

    #[test]
    fn part1_test3() {
        let code = vec![0,1,5,4,3,0];
        let mut hal = Computer::new(2024, 0, 0, code);
        hal.exec();
        assert_eq!(hal.stdout, "4,2,5,6,7,7,7,7,3,1,0");
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
        assert_eq!(hal.stdout, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_test1() {
        let code = vec![0,3,5,4,3,0];
        let mut hal = Computer::new(117440, 0, 0, code);
        hal.exec();
        assert_eq!(hal.stdout, "0,3,5,4,3,0");
    }

}