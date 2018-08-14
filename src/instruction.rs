#[derive(Debug, PartialEq)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    JMP,
    IGL,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode: opcode
        }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::LOAD,
            1 => return Opcode::ADD,
            2 => return Opcode::SUB,
            3 => return Opcode::MUL,
            4 => return Opcode::DIV,
            5 => return Opcode::HLT,
            6 => return Opcode::JMP,
            _ => return Opcode::IGL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_load() {
      let instruction = Instruction::new(Opcode::LOAD);
      assert_eq!(instruction.opcode, Opcode::LOAD);
    }

    #[test]
    fn test_create_add() {
      let instruction = Instruction::new(Opcode::ADD);
      assert_eq!(instruction.opcode, Opcode::ADD);
    }

    #[test]
    fn test_create_sub() {
      let instruction = Instruction::new(Opcode::SUB);
      assert_eq!(instruction.opcode, Opcode::SUB);
    }

    #[test]
    fn test_create_mul() {
      let instruction = Instruction::new(Opcode::MUL);
      assert_eq!(instruction.opcode, Opcode::MUL);
    }

    #[test]
    fn test_create_div() {
      let instruction = Instruction::new(Opcode::DIV);
      assert_eq!(instruction.opcode, Opcode::DIV);
    }

    #[test]
    fn test_create_jmp() {
      let instruction = Instruction::new(Opcode::JMP);
      assert_eq!(instruction.opcode, Opcode::JMP);
    }

    #[test]
    fn test_create_instruction() {
      let instruction = Instruction::new(Opcode::HLT);
      assert_eq!(instruction.opcode, Opcode::HLT);
    }

}
