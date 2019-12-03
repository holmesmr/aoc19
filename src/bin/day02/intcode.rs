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
        CPUException { kind, message }
    }

    pub fn out_of_bounds(ident: &str, pos: usize) -> Self {
        CPUException {
            kind: CPUExceptionKind::OutOfBounds,
            message: format!("{}: pos {} is outside program bounds", ident, pos),
        }
    }

    pub fn invalid_opcode(opcode: u32) -> Self {
        CPUException {
            kind: CPUExceptionKind::InvalidOpcode,
            message: format!("Invalid opcode {}", opcode),
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
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!ADD.src1", src1))?;
                let src2_val = *self
                    .program
                    .get(src2)
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!ADD.src2", src2))?;
                let dst_cell = self
                    .program
                    .get_mut(dst)
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!ADD.dst", dst))?;
                *dst_cell = src1_val + src2_val;
            }
            CPUOp::Mul { src1, src2, dst } => {
                let src1_val = *self
                    .program
                    .get(src1)
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!MUL.src1", src1))?;
                let src2_val = *self
                    .program
                    .get(src2)
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!MUL.src2", src2))?;
                let dst_cell = self
                    .program
                    .get_mut(dst)
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!MUL.dst", dst))?;
                *dst_cell = src1_val * src2_val;
            }
            CPUOp::Halt => self.state = CPUState::Halted,
            CPUOp::Undefined(opcode) => return Err(CPUException::invalid_opcode(opcode)),
        }

        self.pc += op.next_pc_offset();
        Ok(())
    }

    fn fetch_op(&mut self) -> CPUResult<CPUOp> {
        use CPUExceptionKind::*;

        let opcode = self
            .program
            .get(self.pc)
            .ok_or_else(|| CPUException::out_of_bounds("FETCH!OP", self.pc))?;

        match opcode {
            1 => {
                let src1 = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!ADD.src1", self.pc + 1))?
                    as usize;
                let src2 = *self
                    .program
                    .get(self.pc + 2)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!ADD.src2", self.pc + 2))?
                    as usize;
                let dst = *self
                    .program
                    .get(self.pc + 3)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!ADD.dst", self.pc + 3))?
                    as usize;

                Ok(CPUOp::Add { src1, src2, dst })
            }
            2 => {
                let src1 = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!MUL.src1", self.pc + 1))?
                    as usize;
                let src2 = *self
                    .program
                    .get(self.pc + 2)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!MUL.src2", self.pc + 2))?
                    as usize;
                let dst = *self
                    .program
                    .get(self.pc + 3)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!MUL.dst", self.pc + 3))?
                    as usize;

                Ok(CPUOp::Mul { src1, src2, dst })
            }
            99 => Ok(CPUOp::Halt),
            undef_op => Ok(CPUOp::Undefined(*undef_op)),
        }
    }

    pub fn step(&mut self) -> CPUResult<CPUState> {
        let op = self.fetch_op()?;
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

    pub fn pc(&self) -> u32 {
        self.pc as u32
    }

    pub fn output(&self) -> u32 {
        *self
            .program
            .get(0)
            .expect("Output (pos 0) not found in program")
    }

    /// noun = input 1 in challenge parlance
    pub fn noun(&self) -> u32 {
        *self
            .program
            .get(1)
            .expect("Noun (pos 1) not found in program")
    }

    /// verb = input 2 in challenge parlance
    pub fn verb(&self) -> u32 {
        *self
            .program
            .get(2)
            .expect("Verb (pos 2) not found in program")
    }

    pub fn inspect_state(&self) -> &[u32] {
        &*self.program
    }
}
