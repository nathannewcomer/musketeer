struct Registers {
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

const SHIFT_MASK: u32 = 0b0001_1111;

const OPCODE: u32 =                 0b0000_0000_0000_0000_0000_0000_0111_1111;
const REGISTER_DESTINATION: u32 =   0b0000_0000_0000_0000_0000_1111_1000_0000;
const FUNCT_3: u32 =                 0b0000_0000_0000_0000_0111_0000_0000_0000;
const REGISTER_1: u32 =             0b0000_0000_0000_1111_1000_0000_0000_0000;
const REGISTER_2: u32 =             0b0000_0001_1111_0000_0000_0000_0000_0000;
const FUNCT_7: u32 =                0b1111_1110_0000_0000_0000_0000_0000_0000;

fn parse_instruction(instruction: u32) {
    // R - type
    let opcode: u32 = instruction & OPCODE;
    let rd: u32 = instruction & REGISTER_DESTINATION;
    let funct3: u32 = instruction & FUNCT_3;
    let rs1: u32 = instruction & REGISTER_1;
    let rs2: u32 = instruction & REGISTER_2;
    let funct7: u32 = instruction & FUNCT_7;


}

// add
fn add(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 + rs2;
}

// subtract
fn sub(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 - rs2;
}

// exclusive or
fn xor(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 ^ rs2;
}

// or
fn or(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 | rs2;
}

// and
fn and(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 & rs2;
}

// shift left logical
fn sll(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 << (rs2 & SHIFT_MASK);
}

// shift right logical
fn srl(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 >> (rs2 & SHIFT_MASK);
}

// shift right arithmetic
fn sra(rd: &mut u32, rs1: u32, rs2: u32) {
    // convert to i32 to use Rust's arithmetic right shift
    let x19 = rs1 as u32;
    let x20  = (rs2 & SHIFT_MASK) as u32;
    *rd = x19 >> x20;
}

// set less than
fn slt(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = match rs1 < rs2 {
        true => 1,
        false => 0
    }
}

// set less than unsigned
fn sltu(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = match rs1 < rs2 {
        true => 1,
        false => 0
    }
}

// ADDI - Add immediate
fn addi(rd: &mut u32, rs1: u32, imm: u32) {
    *rd = rs1 + imm;
}

// xori - xor immediate
fn xori(rd: &mut u32, rs1: u32, imm: u32) {
    *rd = rs1 ^ imm;
}

// ori - xor immediate
fn ori(rd: &mut u32, rs1: u32, imm: u32) {
    *rd = rs1 | imm;
}

// and immediate
fn andi(rd: &mut u32, rs1: u32, imm: u32) {
    *rd = rs1 & imm;
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

fn create_registers() -> Registers {
    Registers {
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

// ADD
#[test]
fn add_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 74336;

    registers.x18 = 384;
    registers.x19 = 73952;

    add(&mut registers.x20, registers.x18, registers.x19);
    assert_eq!(registers.x20, expected_value);
}

// SUB
#[test]
fn sub_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 7;

    registers.x18 = 12;
    registers.x19 = 5;

    sub(&mut registers.x20, registers.x18, registers.x19);
    assert_eq!(registers.x20, expected_value);
}

// XOR
#[test]
fn xor_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b1111_1111_1001;

    registers.x18 = 0b0000_1111_1010;
    registers.x19 = 0b1111_0000_0011;

    xor(&mut registers.x20, registers.x18, registers.x19);
    assert_eq!(registers.x20, expected_value);
}

// OR
#[test]
fn or_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b1111_1111_1011;

    registers.x18 = 0b0000_1111_1010;
    registers.x19 = 0b1111_0000_0011;

    or(&mut registers.x20, registers.x18, registers.x19);
    assert_eq!(registers.x20, expected_value);
}

// AND
#[test]
fn and_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b0000_0000_0010;

    registers.x18 = 0b0000_1111_1010;
    registers.x19 = 0b1111_0000_0011;

    and(&mut registers.x20, registers.x18, registers.x19);
    assert_eq!(registers.x20, expected_value);
}

// SLL
#[test]
fn sll_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 32;

    registers.x18 = 16;
    registers.x19 = 1;

    sll(&mut registers.x20, registers.x18, registers.x19);
    assert_eq!(registers.x20, expected_value);
}

#[test]
fn sll_test_2() {
    let mut registers = create_registers();
    let expected_value: u32 = 0;

    registers.x18 = 0b1000_0000_0000_0000;
    registers.x19 = 24;

    sll(&mut registers.x20, registers.x18, registers.x19);
    assert_eq!(registers.x20, expected_value);
}

// SRL
#[test]
fn srl_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;

    registers.x18 = 0b0000_0000_0000_1000_0000_0000_0000_0000;
    registers.x19 = 4;

    srl(&mut registers.x20, registers.x18, registers.x19);
    assert_eq!(registers.x20, expected_value);
}

#[test]
fn srl_test_2() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b0000_1000_0000_0000_0000_0000_0000_0000_u32;

    registers.x18 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    registers.x19 = 4;

    srl(&mut registers.x20, registers.x18, registers.x19);
    assert_eq!(registers.x20, expected_value);
}

// SRA
#[test]
fn sra_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;

    registers.x18 = 0b0000_0000_0000_1000_0000_0000_0000_0000;
    registers.x19 = 4;

    sra(&mut registers.x20, registers.x18, registers.x19);
    assert_eq!(registers.x20, expected_value);
}

// SLT

// SLTU