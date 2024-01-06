const SHIFT_MASK: u32 = 0b0001_1111;

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

pub struct CPU {
    x0: u32,                // zero constant
    x1: u32,                // return address
    x2: u32,                // stack pointer
    x3: u32,                // global pointer
    x4: u32,                // thread pointer
    x5: u32,                // temporary
    x6: u32,                // temporary
    x7: u32,                // temporary
    x8: u32,                // saved/frame pointer
    x9: u32,                // saved register
    x10: u32,               // function args/return values
    x11: u32,               // function args/return values
    x12: u32,               // function args
    x13: u32,               // function args
    x14: u32,               // function args
    x15: u32,               // function args
    x16: u32,               // function args
    x17: u32,               // function args
    x18: u32,               // saved registers
    x19: u32,               // saved registers
    x20: u32,               // saved registers
    x21: u32,               // saved registers
    x22: u32,               // saved registers
    x23: u32,               // saved registers
    x24: u32,               // saved registers
    x25: u32,               // saved registers
    x26: u32,               // saved registers
    x27: u32,               // saved registers
    x28: u32,               // temporaries
    x29: u32,               // temporaries
    x30: u32,               // temporaries
    x31: u32,               // temporaries
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            x0: 0,
            x1: 0,
            x2: 0,
            x3: 0,
            x4: 0,
            x5: 0,
            x6: 0,
            x7: 0,
            x8: 0,
            x9: 0,
            x10: 0,
            x11: 0,
            x12: 0,
            x13: 0,
            x14: 0,
            x15: 0,
            x16: 0,
            x17: 0,
            x18: 0,
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            x30: 0,
            x31: 0,
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
        let rd: u32 = instruction & REGISTER_DESTINATION;
        let funct3: u32 = instruction & FUNCT_3;
        let register_1: u32 = instruction & REGISTER_1;
        let register_2: u32 = instruction & REGISTER_2;
        let funct7: u32 = instruction & FUNCT_7;
    
        // get register references from instruction
        let rs1 = self.get_reg_val(register_1);
        let rs2 = self.get_reg_val(register_2);
    
        match (funct3, funct7) {
            (FUNCT_7_ADD, FUNCT_7_ADD) => {
                self.add(rd, rs1, rs2)
            },
            _ => ()
        }
    }

    fn set_register(&mut self, rs: u32, value: u32) {
        match rs {
            0 => self.x0 = value,
            1 => self.x1 = value,
            2 => self.x2 = value,
            3 => self.x3 = value,
            4 => self.x4 = value,
            5 => self.x5 = value,
            6 => self.x6 = value,
            7 => self.x7 = value,
            8 => self.x8 = value,
            9 => self.x9 = value,
            10 => self.x10 = value,
            11 => self.x11 = value,
            12 => self.x12 = value,
            13 => self.x13 = value,
            14 => self.x14 = value,
            15 => self.x15 = value,
            16 => self.x16 = value,
            17 => self.x17 = value,
            18 => self.x18 = value,
            19 => self.x19 = value,
            20 => self.x20 = value,
            21 => self.x21 = value,
            22 => self.x22 = value,
            23 => self.x23 = value,
            24 => self.x24 = value,
            25 => self.x25 = value,
            26 => self.x26 = value,
            27 => self.x27 = value,
            28 => self.x28 = value,
            29 => self.x29 = value,
            30 => self.x30 = value,
            31 => self.x31 = value,
            _ => panic!("{rs} is not a valid register.")
        }
    }

    fn get_reg_val(&self, rs: u32) -> u32 {
        match rs {
            0 => self.x0,
            1 => self.x1,
            2 => self.x2,
            3 => self.x3,
            4 => self.x4,
            5 => self.x5,
            6 => self.x6,
            7 => self.x7,
            8 => self.x8,
            9 => self.x9,
            10 => self.x10,
            11 => self.x11,
            12 => self.x12,
            13 => self.x13,
            14 => self.x14,
            15 => self.x15,
            16 => self.x16,
            17 => self.x17,
            18 => self.x18,
            19 => self.x19,
            20 => self.x20,
            21 => self.x21,
            22 => self.x22,
            23 => self.x23,
            24 => self.x24,
            25 => self.x25,
            26 => self.x26,
            27 => self.x27,
            28 => self.x28,
            29 => self.x29,
            30 => self.x30,
            31 => self.x31,
            _ => panic!("{rs} is not a valid register.")
        }
    }

    // add
    fn add(&mut self, rd: u32, rs1: u32, rs2: u32) {
        let result = self.get_reg_val(rs1)
            + self.get_reg_val(rs2);
        self.set_register(rd, result);
    }

    // subtract
    fn sub(&mut self, rd: u32, rs1: u32, rs2: u32) {
        let result = self.get_reg_val(rs1)
            - self.get_reg_val(rs2);
        self.set_register(rd, result);
    }

    // exclusive or
    fn xor(&mut self, rd: u32, rs1: u32, rs2: u32) {
        let result = self.get_reg_val(rs1)
            ^ self.get_reg_val(rs2);
        self.set_register(rd, result);
    }

// or
    fn or(&mut self, rd: u32, rs1: u32, rs2: u32) {
        let result = self.get_reg_val(rs1)
            | self.get_reg_val(rs2);
        self.set_register(rd, result);
    }

    // and
    fn and(&mut self, rd: u32, rs1: u32, rs2: u32) {
        let result = self.get_reg_val(rs1)
            & self.get_reg_val(rs2);
        self.set_register(rd, result);
    }

    // shift left logical
    fn sll(&mut self, rd: u32, rs1: u32, rs2: u32) {
        let result = self.get_reg_val(rs1)
            << self.get_reg_val(rs2 & SHIFT_MASK);
        self.set_register(rd, result);
    }

    // shift right logical
    fn srl(&mut self, rd: u32, rs1: u32, rs2: u32) {
        let result = self.get_reg_val(rs1)
            >> self.get_reg_val(rs2 & SHIFT_MASK);
        self.set_register(rd, result);
    }

    // shift right arithmetic
    fn sra(&mut self, rd: u32, rs1: u32, rs2: u32) {
        // convert to i32 to use Rust's arithmetic right shift
        let result = self.get_reg_val(rs1) as i32
            >> self.get_reg_val(rs2 & SHIFT_MASK) as i32;
        self.set_register(rd, result as u32);
    }

    // set less than
    fn slt(&mut self, rd: u32, rs1: u32, rs2: u32) {
        let result = match (self.get_reg_val(rs1) as i32)
                < (self.get_reg_val(rs2) as i32) {
            true => 1,
            false => 0
        };

        self.set_register(rd, result);
    }

    // set less than unsigned
    fn sltu(&mut self, rd: u32, rs1: u32, rs2: u32) {
        let result = match self.get_reg_val(rs1)
                < self.get_reg_val(rs2) {
            true => 1,
            false => 0
        };

        self.set_register(rd, result);
    }

    // add immediate
    fn addi(&mut self, rd: u32, rs1: u32, imm: u32) {
        let result = self.get_reg_val(rs1)
            + imm;
        self.set_register(rd, result);
    }

    // xori - xor immediate
    fn xori(&mut self, rd: u32, rs1: u32, imm: u32) {
        let result = self.get_reg_val(rs1)
            ^ imm;
        self.set_register(rd, result);
    }

    // ori - xor immediate
    fn ori(&mut self, rd: u32, rs1: u32, imm: u32) {
        let result = self.get_reg_val(rs1)
            + imm;
        self.set_register(rd, result);
    }

    // and immediate
    fn andi(&mut self, rd: u32, rs1: u32, imm: u32) {
        let result = self.get_reg_val(rs1)
            | imm;
        self.set_register(rd, result);
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

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 384;
    cpu.x19 = 73952;


    cpu.add(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

// SUB
#[test]
fn sub_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 7;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 12;
    cpu.x19 = 5;


    cpu.sub(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

// XOR
#[test]
fn xor_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b1111_1111_1001;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 0b0000_1111_1010;
    cpu.x19 = 0b1111_0000_0011;


    cpu.xor(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

// OR
#[test]
fn or_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b1111_1111_1011;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 0b0000_1111_1010;
    cpu.x19 = 0b1111_0000_0011;


    cpu.or(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

// AND
#[test]
fn and_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b0000_0000_0010;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 0b0000_1111_1010;
    cpu.x19 = 0b1111_0000_0011;


    cpu.and(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

// SLL
#[test]
fn sll_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 32;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 16;
    cpu.x19 = 1;


    cpu.sll(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

#[test]
fn sll_test_2() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 0b1000_0000_0000_0000;
    cpu.x19 = 24;


    cpu.sll(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

// SRL
#[test]
fn srl_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 0b0000_0000_0000_1000_0000_0000_0000_0000;
    cpu.x19 = 4;


    cpu.srl(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

#[test]
fn srl_test_2() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b0000_1000_0000_0000_0000_0000_0000_0000_u32;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    cpu.x19 = 4;


    cpu.srl(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

// SRA
#[test]
fn sra_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 0b0000_0000_0000_1000_0000_0000_0000_0000;
    cpu.x19 = 4;


    cpu.srl(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

// SLT
#[test]
fn slt_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 1;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 9;
    cpu.x19 = 87;


    cpu.slt(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

#[test]
fn slt_test_2() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 0;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 9;
    cpu.x19 = 0xFF_FF_FF_A9;            // 87 in two's complement


    cpu.slt(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

// SLTU
#[test]
fn sltu_test_1() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 1;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 9;
    cpu.x19 = 87;


    cpu.sltu(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
}

#[test]
fn sltu_test_2() {
    let mut cpu = CPU::new();
    let expected_value: u32 = 1;

    let rd: u32 = 20;
    let rs1: u32 = 18;
    let rs2: u32 = 19;

    cpu.x18 = 9;
    cpu.x19 = 0xFF_FF_FF_A9;            // 87 in two's complement


    cpu.sltu(rd, rs1, rs2);
    assert_eq!(cpu.x20, expected_value);
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