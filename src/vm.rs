use instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: usize,
    equal_flag: bool
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        let opcode = self.decode_opcode();
        match opcode {
            Opcode::LOAD => {
                let register = self.next_byte() as usize;
                let number = self.next_two_bytes() as u32;
                self.registers[register] = number as i32;
            },
            Opcode::ADD => {
                let register_a = self.registers[self.next_byte() as usize];
                let register_b = self.registers[self.next_byte() as usize];
                let result_register = self.next_byte() as usize;
                self.registers[result_register] = register_a + register_b;
            },
            Opcode::SUB => {
                let register_a = self.registers[self.next_byte() as usize];
                let register_b = self.registers[self.next_byte() as usize];
                let result_register = self.next_byte() as usize;
                self.registers[result_register] = register_a - register_b;
            },
            Opcode::MUL => {
                let register_a = self.registers[self.next_byte() as usize];
                let register_b = self.registers[self.next_byte() as usize];
                let result_register = self.next_byte() as usize;
                self.registers[result_register] = register_a * register_b;
            },
            Opcode::DIV => {
                let register_a = self.registers[self.next_byte() as usize];
                let register_b = self.registers[self.next_byte() as usize];
                let result_register = self.next_byte() as usize;
                self.registers[result_register] = register_a / register_b;
                self.remainder = (register_a % register_b) as usize;
            },
            Opcode::JMP => {
                let target = self.registers[self.next_byte() as usize];
                self.pc = target as usize;
            },
            Opcode::JMPF => {
                let value = self.registers[self.next_byte() as usize] as usize;
                self.pc += value;
            },
            Opcode::JMPB => {
                let value = self.registers[self.next_byte() as usize] as usize;
                self.pc -= value;
            },
            Opcode::JMPE => {
                let target = self.registers[self.next_byte() as usize] as usize;
                self.pc += 2;
                if self.equal_flag {
                    self.pc = target as usize;
                }
            },
            Opcode::EQ => {
                let register_a = self.registers[self.next_byte() as usize];
                let register_b = self.registers[self.next_byte() as usize];
                self.equal_flag = register_a == register_b;
                self.pc += 1
            },
            Opcode::HLT => {
                println!("HLT encountered");
                false;
            },
            _ => {
                println!("Unrecognized opcode found! Terminating!");
                false;
            }
        }
        true
    }

    fn decode_opcode(&mut self) -> Opcode {
        let result = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return result;
    }

    fn next_byte(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }
    fn next_two_bytes(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![0, 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![2, 1, 0, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 50);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![4, 1, 0, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 5, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[1] = 6;
        test_vm.program = vec![0, 0, 0, 10, 8, 1, 0, 0];
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![9, 0, 0, 0, 200, 0, 0, 0, 200, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }
}
