use instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0
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
}
