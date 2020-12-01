use std::str::FromStr;

enum Operand {
    Position(usize),
    Immediate(i32),
}

impl Operand {
    fn new(mode: char, value: i32) -> CPUResult<Operand> {
        match mode {
            '0' => Ok(Operand::Position(value as usize)),
            '1' => Ok(Operand::Immediate(value)),
            _ => Err(CPUException::invalid_operand(mode)),
        }
    }
}

enum CPUOp {
    Add {
        src1: Operand,
        src2: Operand,
        dst: usize,
    },
    Mul {
        src1: Operand,
        src2: Operand,
        dst: usize,
    },
    Halt,
    Input(usize),
    Output(Operand),
    JumpZero {
        cmp: Operand,
        to: Operand,
    },
    JumpNonZero {
        cmp: Operand,
        to: Operand,
    },
    CompareLess {
        cmp1: Operand,
        cmp2: Operand,
        dst: usize,
    },
    CompareEqual {
        cmp1: Operand,
        cmp2: Operand,
        dst: usize,
    },
    Undefined(i32),
}

impl CPUOp {
    fn next_pc_offset(&self) -> usize {
        match *self {
            CPUOp::Add { .. }
            | CPUOp::Mul { .. }
            | CPUOp::CompareEqual { .. }
            | CPUOp::CompareLess { .. } => 4,
            CPUOp::JumpZero { .. } | CPUOp::JumpNonZero { .. } => 3,
            CPUOp::Input(_) | CPUOp::Output(_) => 2,
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
    InvalidOperand,
    InvalidInput,
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

    pub fn invalid_opcode(opcode: i32) -> Self {
        CPUException {
            kind: CPUExceptionKind::InvalidOpcode,
            message: format!("Invalid opcode {}", opcode),
        }
    }

    pub fn invalid_operand(operand: char) -> Self {
        CPUException {
            kind: CPUExceptionKind::InvalidOperand,
            message: format!("Invalid operand mode {}", operand),
        }
    }
}

pub type CPUResult<T> = Result<T, CPUException>;

pub struct IntcodeCPU {
    program: Vec<i32>,
    state: CPUState,
    pc: usize,
}

impl IntcodeCPU {
    pub fn new(program: Vec<i32>) -> Self {
        IntcodeCPU {
            program,
            state: CPUState::Running,
            pc: 0,
        }
    }

    fn get_operand_value(&self, oper: Operand, ident: &str) -> CPUResult<i32> {
        use Operand::*;

        match oper {
            Position(idx) => self
                .program
                .get(idx)
                .ok_or_else(|| CPUException::out_of_bounds(ident, idx))
                .map(|&v| v),
            Immediate(val) => Ok(val),
        }
    }

    fn execute_op(&mut self, op: CPUOp) -> CPUResult<()> {
        use CPUExceptionKind::*;

        let offset = op.next_pc_offset();
        match op {
            CPUOp::Add { src1, src2, dst } => {
                let src1_val = self.get_operand_value(src1, "EXEC!ADD.src1")?;
                let src2_val = self.get_operand_value(src2, "EXEC!ADD.src2")?;
                let dst_cell = self
                    .program
                    .get_mut(dst)
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!ADD.dst", dst))?;
                *dst_cell = src1_val + src2_val;
            }
            CPUOp::Mul { src1, src2, dst } => {
                let src1_val = self.get_operand_value(src1, "EXEC!MUL.src1")?;
                let src2_val = self.get_operand_value(src2, "EXEC!MUL.src2")?;
                let dst_cell = self
                    .program
                    .get_mut(dst)
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!MUL.dst", dst))?;
                *dst_cell = src1_val * src2_val;
            }
            CPUOp::Halt => self.state = CPUState::Halted,
            CPUOp::Input(dst) => {
                use std::io::Write;
                let dst_cell = self
                    .program
                    .get_mut(dst)
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!INPUT.dst", dst))?;

                let mut s = String::new();
                print!("Input value: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut s).map_err(|_| {
                    CPUException::new(
                        CPUExceptionKind::InvalidInput,
                        "Could not read input".into(),
                    )
                })?;
                let input = i32::from_str(&*s.trim()).map_err(|_| {
                    CPUException::new(
                        CPUExceptionKind::InvalidInput,
                        format!("Could not parse {} as i32", &*s.trim()),
                    )
                })?;

                *dst_cell = input;
            }
            CPUOp::JumpZero { cmp, to } => {
                let cmp = self.get_operand_value(cmp, "EXEC!JZ.cmp")?;
                let to = self.get_operand_value(to, "EXEC!JZ.to")? as usize;

                if cmp == 0 {
                    self.pc = to;
                    return Ok(());
                }
            }
            CPUOp::JumpNonZero { cmp, to } => {
                let cmp = self.get_operand_value(cmp, "EXEC!JNZ.cmp")?;
                let to = self.get_operand_value(to, "EXEC!JNZ.to")? as usize;

                if cmp != 0 {
                    self.pc = to;
                    return Ok(());
                }
            }
            CPUOp::CompareEqual { cmp1, cmp2, dst } => {
                let cmp1 = self.get_operand_value(cmp1, "EXEC!EQ.cmp1")?;
                let cmp2 = self.get_operand_value(cmp2, "EXEC!EQ.cmp2")?;
                let dst_cell = self
                    .program
                    .get_mut(dst)
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!EQ.dst", dst))?;

                if cmp1 == cmp2 {
                    *dst_cell = 1;
                } else {
                    *dst_cell = 0;
                }
            }
            CPUOp::CompareLess { cmp1, cmp2, dst } => {
                let cmp1 = self.get_operand_value(cmp1, "EXEC!LT.cmp1")?;
                let cmp2 = self.get_operand_value(cmp2, "EXEC!LT.cmp2")?;
                let dst_cell = self
                    .program
                    .get_mut(dst)
                    .ok_or_else(|| CPUException::out_of_bounds("EXEC!LT.dst", dst))?;

                if cmp1 < cmp2 {
                    *dst_cell = 1;
                } else {
                    *dst_cell = 0;
                }
            }
            CPUOp::Output(src) => println!(
                "Program output: {}",
                self.get_operand_value(src, "EXEC!OUTPUT.src")?
            ),
            CPUOp::Undefined(opcode) => return Err(CPUException::invalid_opcode(opcode)),
        }

        self.pc += offset;
        Ok(())
    }

    fn fetch_op(&mut self) -> CPUResult<CPUOp> {
        use CPUExceptionKind::*;

        let opcode = self
            .program
            .get(self.pc)
            .ok_or_else(|| CPUException::out_of_bounds("FETCH!OP", self.pc))?;

        if *opcode < 0 {
            return Err(CPUException::invalid_opcode(*opcode));
        }

        let opcode_str = format!("{:05}", opcode);

        let (operand_modes, op) = opcode_str.split_at(3);
        let operand_modes = operand_modes.chars().rev().collect::<Vec<char>>();

        match op {
            "01" => {
                let src1 = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!ADD.src1", self.pc + 1))?;
                let src2 = *self
                    .program
                    .get(self.pc + 2)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!ADD.src2", self.pc + 2))?;
                let dst = *self
                    .program
                    .get(self.pc + 3)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!ADD.dst", self.pc + 3))?
                    as usize;

                Ok(CPUOp::Add {
                    src1: Operand::new(operand_modes[0], src1)?,
                    src2: Operand::new(operand_modes[1], src2)?,
                    dst,
                })
            }
            "02" => {
                let src1 = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!MUL.src1", self.pc + 1))?;
                let src2 = *self
                    .program
                    .get(self.pc + 2)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!MUL.src2", self.pc + 2))?;
                let dst = *self
                    .program
                    .get(self.pc + 3)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!MUL.dst", self.pc + 3))?
                    as usize;

                Ok(CPUOp::Mul {
                    src1: Operand::new(operand_modes[0], src1)?,
                    src2: Operand::new(operand_modes[1], src2)?,
                    dst,
                })
            }
            "03" => {
                let dst =
                    *self.program.get(self.pc + 3).ok_or_else(|| {
                        CPUException::out_of_bounds("FETCH!INPUT.dst", self.pc + 1)
                    })? as usize;

                Ok(CPUOp::Input(dst))
            }
            "04" => {
                let src = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!OUTPUT.src", self.pc + 1))?;

                Ok(CPUOp::Output(Operand::new(operand_modes[0], src)?))
            }
            "05" => {
                let cmp = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!JNZ.cmp", self.pc + 1))?;
                let to = *self
                    .program
                    .get(self.pc + 2)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!JNZ.to", self.pc + 2))?;

                Ok(CPUOp::JumpNonZero {
                    cmp: Operand::new(operand_modes[0], cmp)?,
                    to: Operand::new(operand_modes[1], to)?,
                })
            }
            "06" => {
                let cmp = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!JZ.cmp", self.pc + 1))?;
                let to = *self
                    .program
                    .get(self.pc + 2)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!JZ.to", self.pc + 2))?;

                Ok(CPUOp::JumpZero {
                    cmp: Operand::new(operand_modes[0], cmp)?,
                    to: Operand::new(operand_modes[1], to)?,
                })
            }
            "07" => {
                let cmp1 = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!LT.cmp1", self.pc + 1))?;
                let cmp2 = *self
                    .program
                    .get(self.pc + 2)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!LT.cmp1", self.pc + 2))?;
                let dst = *self
                    .program
                    .get(self.pc + 3)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!LT.dst", self.pc + 3))?
                    as usize;

                Ok(CPUOp::CompareLess {
                    cmp1: Operand::new(operand_modes[0], cmp1)?,
                    cmp2: Operand::new(operand_modes[1], cmp2)?,
                    dst,
                })
            }
            "08" => {
                let cmp1 = *self
                    .program
                    .get(self.pc + 1)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!EQ.cmp1", self.pc + 1))?;
                let cmp2 = *self
                    .program
                    .get(self.pc + 2)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!EQ.cmp1", self.pc + 2))?;
                let dst = *self
                    .program
                    .get(self.pc + 3)
                    .ok_or_else(|| CPUException::out_of_bounds("FETCH!EQ.dst", self.pc + 3))?
                    as usize;

                Ok(CPUOp::CompareEqual {
                    cmp1: Operand::new(operand_modes[0], cmp1)?,
                    cmp2: Operand::new(operand_modes[1], cmp2)?,
                    dst,
                })
            }
            "99" => Ok(CPUOp::Halt),
            undef_op => Ok(CPUOp::Undefined(i32::from_str(undef_op).unwrap())),
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

    pub fn get_position(&self, pos: usize) -> Option<i32> {
        self.program.get(pos).cloned()
    }

    pub fn pc(&self) -> u32 {
        self.pc as u32
    }

    pub fn output(&self) -> i32 {
        *self
            .program
            .get(0)
            .expect("Output (pos 0) not found in program")
    }

    /// noun = input 1 in challenge parlance
    pub fn noun(&self) -> i32 {
        *self
            .program
            .get(1)
            .expect("Noun (pos 1) not found in program")
    }

    /// verb = input 2 in challenge parlance
    pub fn verb(&self) -> i32 {
        *self
            .program
            .get(2)
            .expect("Verb (pos 2) not found in program")
    }

    pub fn inspect_state(&self) -> &[i32] {
        &*self.program
    }
}
