const SHIFT_MASK: usize = 0b0001_1111;

// instruction masks
// R-type
const OPCODE: u32 =                 0b0000_0000_0000_0000_0000_0000_0111_1111;
const REGISTER_DESTINATION: u32 =   0b0000_0000_0000_0000_0000_1111_1000_0000;
const FUNCT_3: u32 =                0b0000_0000_0000_0000_0111_0000_0000_0000;
const REGISTER_1: u32 =             0b0000_0000_0000_1111_1000_0000_0000_0000;
const REGISTER_2: u32 =             0b0000_0001_1111_0000_0000_0000_0000_0000;
const FUNCT_7: u32 =                0b1111_1110_0000_0000_0000_0000_0000_0000;

// add
const FUNCT_3_ADD: u32 =                0b0000_0000_0000_0000_0000_0000_0000_0000;
const FUNCT_7_ADD: u32 =                0b0000_0000_0000_0000_0000_0000_0000_0000;

// add x2, x4, x5
// 00000000010100100000000100110011

// x0 - zero constant
// x1 - return address
// x2 - stack pointer
// x3 - global pointer
// x4 - thread pointer
// x5 - temporary
// x6 - temporary
// x7 - temporary
// x8 - saved/frame pointer
// x9 - saved register
// x10 - function args/return values
// x11 - function args/return values
// x12 - function args
// x13 - function args
// x14 - function args
// x15 - function args
// x16 - function args
// x17 - function args
// x18 - saved registers
// x19 - saved registers
// x20 - saved registers
// x21 - saved registers
// x22 - saved registers
// x23 - saved registers
// x24 - saved registers
// x25 - saved registers
// x26 - saved registers
// x27 - saved registers
// x28 - temporaries
// x29 - temporaries
// x30 - temporaries
// x31 - temporaries

pub struct CPU {
    registers: [u32; 32]
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: [0; 32]
        }
    }

    // Parses an instruction from bytes.
    pub fn parse_instruction(&mut self, instruction: u32) {
        // R - type
        let opcode: u32 = instruction & OPCODE;
    
        match opcode {
            0b0110011 => self.parse_rtype(instruction),      // R-type
            _ => ()
        }
    }

    fn parse_rtype(&mut self, instruction: u32) {
        // break up instruction into sections
        let rd: usize = (instruction & REGISTER_DESTINATION).try_into().unwrap();;
        let funct3: u32 = instruction & FUNCT_3;
        let rs1: usize = (instruction & REGISTER_1).try_into().unwrap();
        let rs2: usize = (instruction & REGISTER_2).try_into().unwrap();
        let funct7: u32 = instruction & FUNCT_7;
    
        match (funct3, funct7) {
            (FUNCT_7_ADD, FUNCT_7_ADD) => {
                self.add(rd, rs1, rs2)
            },
            _ => ()
        }
    }

    // add
    fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let result = self.registers[rs1]
            + self.registers[rs2];
        self.registers[rd] = result
    }

    // subtract
    fn sub(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let result = self.registers[rs1]
            - self.registers[rs2];
        self.registers[rd] = result
    }

    // exclusive or
    fn xor(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let result = self.registers[rs1]
            ^ self.registers[rs2];
        self.registers[rd] = result
    }

// or
    fn or(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let result = self.registers[rs1]
            | self.registers[rs2];
        self.registers[rd] = result
    }

    // and
    fn and(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let result = self.registers[rs1]
            & self.registers[rs2];
        self.registers[rd] = result
    }

    // shift left logical
    fn sll(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let result = self.registers[rs1]
            << self.registers[rs2 & SHIFT_MASK];
        self.registers[rd] = result
    }

    // shift right logical
    fn srl(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let result = self.registers[rs1]
            >> self.registers[rs2 & SHIFT_MASK];
        self.registers[rd] = result
    }

    // shift right arithmetic
    fn sra(&mut self, rd: usize, rs1: usize, rs2: usize) {
        // convert to i32 to use Rust's arithmetic right shift
        let result = self.registers[rs1] as i32
            >> self.registers[rs2 & SHIFT_MASK] as i32;
        self.registers[rd]=  result as u32;
    }

    // set less than
    fn slt(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let result = match (self.registers[rs1] as i32)
                < (self.registers[rs2] as i32) {
            true => 1,
            false => 0
        };

        self.registers[rd] = result
    }

    // set less than unsigned
    fn sltu(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let result = match self.registers[rs1]
                < self.registers[rs2] {
            true => 1,
            false => 0
        };

        self.registers[rd] = result
    }

    // add immediate
    fn addi(&mut self, rd: usize, rs1: usize, imm: u32) {
        let result = self.registers[rs1]
            + imm;
        self.registers[rd] = result
    }

    // xori - xor immediate
    fn xori(&mut self, rd: usize, rs1: usize, imm: u32) {
        let result = self.registers[rs1]
            ^ imm;
        self.registers[rd] = result
    }

    // ori - xor immediate
    fn ori(&mut self, rd: usize, rs1: usize, imm: u32) {
        let result = self.registers[rs1]
            + imm;
        self.registers[rd] = result
    }

    // and immediate
    fn andi(&mut self, rd: usize, rs1: usize, imm: u32) {
        let result = self.registers[rs1]
            | imm;
        self.registers[rd] = result
    }

    // shift left logical immediate
    fn slli() {

    }

    // shift right logical immediate
    fn srli() {

    }

    // shift right arithmetic immediate
    fn srai() {

    }

    // set less than immediate
    fn slti() {

    }

    // set less than immediate unsigned
    fn sltiu() {

    }

    // ---------- I-type ----------

    // load byte
    fn lb(&self, rd: &mut u32, imm: u32) {

    }
}

// https://luplab.gitlab.io/rvcodecjs/#q=add+x2,+x4,+x5&abi=false&isa=AUTO
// use to assemble instructions

// -------------------- Arithmetic --------------------

// -------------------- Storage -----------------------



// ADD
#[test]
fn add_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 74336;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 384;
    cpu.registers[19] = 73952;


    cpu.add(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

// SUB
#[test]
fn sub_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 7;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 12;
    cpu.registers[19] = 5;


    cpu.sub(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

// XOR
#[test]
fn xor_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b1111_1111_1001;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 0b0000_1111_1010;
    cpu.registers[19] = 0b1111_0000_0011;


    cpu.xor(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

// OR
#[test]
fn or_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b1111_1111_1011;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 0b0000_1111_1010;
    cpu.registers[19] = 0b1111_0000_0011;


    cpu.or(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

// AND
#[test]
fn and_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b0000_0000_0010;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 0b0000_1111_1010;
    cpu.registers[19] = 0b1111_0000_0011;


    cpu.and(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

// SLL
#[test]
fn sll_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 32;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 16;
    cpu.registers[19] = 1;


    cpu.sll(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

#[test]
fn sll_test_2() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 0b1000_0000_0000_0000;
    cpu.registers[19] = 24;


    cpu.sll(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

// SRL
#[test]
fn srl_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 0b0000_0000_0000_1000_0000_0000_0000_0000;
    cpu.registers[19] = 4;


    cpu.srl(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

#[test]
fn srl_test_2() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b0000_1000_0000_0000_0000_0000_0000_0000_u32;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    cpu.registers[19] = 4;


    cpu.srl(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

// SRA
#[test]
fn sra_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 0b0000_0000_0000_1000_0000_0000_0000_0000;
    cpu.registers[19] = 4;


    cpu.srl(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

// SLT
#[test]
fn slt_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 1;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 9;
    cpu.registers[19] = 87;


    cpu.slt(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

#[test]
fn slt_test_2() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 9;
    cpu.registers[19] = 0xFF_FF_FF_A9;            // 87 in two's complement


    cpu.slt(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

// SLTU
#[test]
fn sltu_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 1;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 9;
    cpu.registers[19] = 87;


    cpu.sltu(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

#[test]
fn sltu_test_2() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 1;

    let rd: usize = 20;
    let rs1: usize = 18;
    let rs2: usize = 19;

    cpu.registers[18] = 9;
    cpu.registers[19] = 0xFF_FF_FF_A9;            // 87 in two's complement


    cpu.sltu(rd, rs1, rs2);
    assert_eq!(cpu.registers[20], expected_value);
}

/*
#[test]
fn addi_test_1() {
    let expected: u32 = 4;
    
    // addi x18, x0, 4
    let instruction: u32 = 0b0000_0000_0100_0000_0000_1001_0001_0011;

    let cpu = CPU::new();
    //cpu.addi(rd, rs1, imm);
}
*/