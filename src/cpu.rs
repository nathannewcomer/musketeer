struct Registers {
    pc: u32,
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    r13: u32,
    r14: u32,
}

const SHIFT_MASK: u32 = 0b0001_1111;

// ADD - adds two registers rs1 and rs2, and stores result in rd register.
fn add(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 + rs2;
}

// SUB - Subtracts two registers rs1 and rs2, and stores result in rd register.
fn sub(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 - rs2;
}

// XOR - Perform exlusive OR on two registers rs1 and rs2, and stores result in rd register.
fn xor(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 ^ rs2;
}

// OR - Perform OR on two registers rs1 and rs2, and stores result in rd register.
fn or(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 | rs2;
}

// AND - Perform AND on two registers rs1 and rs2, and stores result in rd register.
fn and(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 & rs2;
}

// SLL - Performs logical left shift on rs1 by rs2 places, and stores result in rd.
fn sll(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 << (rs2 & SHIFT_MASK);
}

// SRL - Performs logical right shift on rs1 by rs2 places, and stores result in rd.
fn srl(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = rs1 >> (rs2 & SHIFT_MASK);
}

// SRA - Performs arithmetic right shift on rs1 by rs2 places, and stores result in rd.
fn sra(rd: &mut u32, rs1: u32, rs2: u32) {
    // convert to i32 to use Rust's arithmetic right shift
    let r1 = rs1 as u32;
    let r2  = (rs2 & SHIFT_MASK) as u32;
    *rd = r1 >> r2;
}

// SLT - Sets rd to 1 if rs1 is less than rs2, 0 otherwise
fn slt(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = match rs1 < rs2 {
        true => 1,
        false => 0
    }
}

// SLTU - Sets rd to 1 if rs1 is less than rs2, 0 otherwise
fn sltu(rd: &mut u32, rs1: u32, rs2: u32) {
    *rd = match rs1 < rs2 {
        true => 1,
        false => 0
    }
}

fn create_registers() -> Registers {
    Registers {
        pc: 0,
        r0: 0,
        r1: 0,
        r2: 0,
        r3: 0,
        r4: 0,
        r5: 0,
        r6: 0,
        r7: 0,
        r8: 0,
        r9: 0,
        r10: 0,
        r11: 0,
        r12: 0,
        r13: 0,
        r14: 0,
    }
}

// ADD
#[test]
fn add_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 74336;

    registers.r0 = 384;
    registers.r1 = 73952;

    add(&mut registers.r2, registers.r0, registers.r1);
    assert_eq!(registers.r2, expected_value);
}

// SUB
#[test]
fn sub_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 7;

    registers.r0 = 12;
    registers.r1 = 5;

    sub(&mut registers.r2, registers.r0, registers.r1);
    assert_eq!(registers.r2, expected_value);
}

// XOR
#[test]
fn xor_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b1111_1111_1001;

    registers.r0 = 0b0000_1111_1010;
    registers.r1 = 0b1111_0000_0011;

    xor(&mut registers.r2, registers.r0, registers.r1);
    assert_eq!(registers.r2, expected_value);
}

// OR
#[test]
fn or_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b1111_1111_1011;

    registers.r0 = 0b0000_1111_1010;
    registers.r1 = 0b1111_0000_0011;

    or(&mut registers.r2, registers.r0, registers.r1);
    assert_eq!(registers.r2, expected_value);
}

// AND
#[test]
fn and_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b0000_0000_0010;

    registers.r0 = 0b0000_1111_1010;
    registers.r1 = 0b1111_0000_0011;

    and(&mut registers.r2, registers.r0, registers.r1);
    assert_eq!(registers.r2, expected_value);
}

// SLL
#[test]
fn sll_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 32;

    registers.r0 = 16;
    registers.r1 = 1;

    sll(&mut registers.r2, registers.r0, registers.r1);
    assert_eq!(registers.r2, expected_value);
}

#[test]
fn sll_test_2() {
    let mut registers = create_registers();
    let expected_value: u32 = 0;

    registers.r0 = 0b1000_0000_0000_0000;
    registers.r1 = 24;

    sll(&mut registers.r2, registers.r0, registers.r1);
    assert_eq!(registers.r2, expected_value);
}

// SRL
#[test]
fn srl_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;

    registers.r0 = 0b0000_0000_0000_1000_0000_0000_0000_0000;
    registers.r1 = 4;

    srl(&mut registers.r2, registers.r0, registers.r1);
    assert_eq!(registers.r2, expected_value);
}

#[test]
fn srl_test_2() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b0000_1000_0000_0000_0000_0000_0000_0000_u32;

    registers.r0 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    registers.r1 = 4;

    srl(&mut registers.r2, registers.r0, registers.r1);
    assert_eq!(registers.r2, expected_value);
}

// SRA
#[test]
fn sra_test_1() {
    let mut registers = create_registers();
    let expected_value: u32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;

    registers.r0 = 0b0000_0000_0000_1000_0000_0000_0000_0000;
    registers.r1 = 4;

    sra(&mut registers.r2, registers.r0, registers.r1);
    assert_eq!(registers.r2, expected_value);
}

// SLT

// SLTU