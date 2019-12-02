enum CPUOp {
    Add {
        src1: usize,
        src2: usize,
        dst: usize,
    },
    Mul {
        src1: usize,
        src2: usize,
        dst: usize,
    },
    Halt,
    Undefined(u32),
}

impl CPUOp {
    fn next_pc_offset(&self) -> usize {
        match *self {
            CPUOp::Add { .. } | CPUOp::Mul { .. } => 4,
            CPUOp::Halt | CPUOp::Undefined { .. } => 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CPUState {
    Running,
    Halted,
}


#[derive(Copy, Clone, Debug)]
pub enum CPUExceptionKind {
    InvalidOpcode,
    OutOfBounds,
}

#[derive(Clone, Debug)]
pub struct CPUException {
    kind: CPUExceptionKind,
    message: String,
}

impl CPUException {
    pub fn new(kind: CPUExceptionKind, message: String) -> Self {
        CPUException {
            kind,
            message
        }
    }
}

pub type CPUResult<T> = Result<T, CPUException>;

pub struct IntcodeCPU {
    program: Vec<u32>,
    state: CPUState,
    pc: usize,
}

impl IntcodeCPU {
    pub fn new(program: Vec<u32>) -> Self {
        IntcodeCPU {
            program,
            state: CPUState::Running,
            pc: 0,
        }
    }

    fn execute_op(&mut self, op: CPUOp) -> CPUResult<()> {
        use CPUExceptionKind::*;

        match op {
            CPUOp::Add { src1, src2, dst } => {
                let src1_val = *self
                    .program
                    .get(src1)
                    .ok_or_else(|| CPUException::new(OutOfBounds, format!("ADD src1 {} is outside program bounds", src1)))?;
                let src2_val = *self
                    .program
                    .get(src2)
                    .ok_or_else(|| CPUException::new(OutOfBounds, format!("ADD src2 {} is outside program bounds", src2)))?;
                let dst_cell = self
                    .program
                    .get_mut(dst)
                    .ok_or_else(|| CPUException::new(OutOfBounds, format!("ADD dest {} is outside program bounds", dst)))?;
                *dst_cell = src1_val + src2_val;
            }
            CPUOp::Mul { src1, src2, dst } => {
                let src1_val = *self
                    .program
                    .get(src1)
                    .ok_or_else(|| CPUException::new(OutOfBounds, format!("MUL src1 {} is outside program bounds", src1)))?;
                let src2_val = *self
                    .program
                    .get(src2)
                    .ok_or_else(|| CPUException::new(OutOfBounds, format!("MUL src2 {} is outside program bounds", src2)))?;
                let dst_cell = self
                    .program
                    .get_mut(dst)
                    .ok_or_else(|| CPUException::new(OutOfBounds, format!("MUL dest {} is outside program bounds", dst)))?;
                *dst_cell = src1_val * src2_val;
            }
            CPUOp::Halt => self.state = CPUState::Halted,
            CPUOp::Undefined(opcode) => return Err(CPUException::new(InvalidOpcode, format!("Invalid opcode {} at position {}", opcode, self.pc))),
        }

        self.pc += op.next_pc_offset();
        Ok(())
    }

    fn load_op(&mut self) -> CPUResult<CPUOp> {
        use CPUExceptionKind::*;

        let opcode = self
            .program
            .get(self.pc)
            .expect("PC fell off end of program while retrieving opcode");

        match opcode {
            1 => {
                let src1 = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::new(OutOfBounds, "PC fell off end of program while retrieving operand src1".into()))?
                    as usize;
                let src2 = *self
                    .program
                    .get(self.pc + 2)
                    .ok_or_else(|| CPUException::new(OutOfBounds, "PC fell off end of program while retrieving operand src2".into()))?
                    as usize;
                let dst = *self
                    .program
                    .get(self.pc + 3)
                    .ok_or_else(|| CPUException::new(OutOfBounds, "PC fell off end of program while retrieving operand dst".into()))?
                    as usize;

                Ok(CPUOp::Add { src1, src2, dst })
            }
            2 => {
                let src1 = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::new(OutOfBounds, "PC fell off end of program while retrieving operand src1".into()))?
                    as usize;
                let src2 = *self
                    .program
                    .get(self.pc + 2)
                    .ok_or_else(|| CPUException::new(OutOfBounds, "PC fell off end of program while retrieving operand src2".into()))?
                    as usize;
                let dst = *self
                    .program
                    .get(self.pc + 3)
                    .ok_or_else(|| CPUException::new(OutOfBounds, "PC fell off end of program while retrieving operand dst".into()))?
                    as usize;

                Ok(CPUOp::Mul { src1, src2, dst })
            }
            99 => Ok(CPUOp::Halt),
            undef_op => Ok(CPUOp::Undefined(*undef_op)),
        }
    }

    pub fn step(&mut self) -> CPUResult<CPUState> {
        let op = self.load_op()?;
        self.execute_op(op)?;

        Ok(self.state)
    }

    pub fn run(&mut self) -> CPUResult<()> {
        loop {
            if let CPUState::Halted = self.step()? {
                return Ok(());
            }
        }
    }

    pub fn get_position(&self, pos: usize) -> Option<u32> {
        self.program.get(pos).cloned()
    }

    pub fn output(&self) -> u32 {
        *self.program.get(0).expect("Output (pos 0) not found in program")
    }

    /// noun = input 1 in challenge parlance
    pub fn noun(&self) -> u32 {
        *self.program.get(1).expect("Noun (pos 1) not found in program")
    }

    /// verb = input 2 in challenge parlance
    pub fn verb(&self) -> u32 {
        *self.program.get(2).expect("Verb (pos 2) not found in program")
    }

    pub fn inspect_state(&self) -> &[u32] {
        &*self.program
    }
}
