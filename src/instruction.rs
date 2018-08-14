#[derive(Debug, PartialEq)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    JMP,
    JMPF,
    JMPB,
    JMPE,
    EQ,
    NEQ,
    GTE,
    LTE,
    GT,
    LT,
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
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            9 => return Opcode::JMPE,
            10 => return Opcode::EQ,
            11 => return Opcode::NEQ,
            12 => return Opcode::GTE,
            13 => return Opcode::LTE,
            14 => return Opcode::GT,
            15 => return Opcode::LT,
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
    fn test_create_jmpf() {
      let instruction = Instruction::new(Opcode::JMPF);
      assert_eq!(instruction.opcode, Opcode::JMPF);
    }

    #[test]
    fn test_create_jmpb() {
      let instruction = Instruction::new(Opcode::JMPB);
      assert_eq!(instruction.opcode, Opcode::JMPB);
    }

    #[test]
    fn test_create_jmpe() {
      let instruction = Instruction::new(Opcode::JMPE);
      assert_eq!(instruction.opcode, Opcode::JMPE);
    }

    #[test]
    fn test_create_eq() {
      let instruction = Instruction::new(Opcode::EQ);
      assert_eq!(instruction.opcode, Opcode::EQ);
    }

    #[test]
    fn test_create_neq() {
      let instruction = Instruction::new(Opcode::NEQ);
      assert_eq!(instruction.opcode, Opcode::NEQ);
    }

    #[test]
    fn test_create_gte() {
      let instruction = Instruction::new(Opcode::GTE);
      assert_eq!(instruction.opcode, Opcode::GTE);
    }

    #[test]
    fn test_create_lte() {
      let instruction = Instruction::new(Opcode::LTE);
      assert_eq!(instruction.opcode, Opcode::LTE);
    }

    #[test]
    fn test_create_gt() {
      let instruction = Instruction::new(Opcode::GT);
      assert_eq!(instruction.opcode, Opcode::GT);
    }

    #[test]
    fn test_create_lt() {
      let instruction = Instruction::new(Opcode::LT);
      assert_eq!(instruction.opcode, Opcode::LT);
    }

    #[test]
    fn test_create_instruction() {
      let instruction = Instruction::new(Opcode::HLT);
      assert_eq!(instruction.opcode, Opcode::HLT);
    }

}
