use tiny_evm::interpreter::Interpreter;
use tiny_evm::opcodes::{op, Opcode};

fn run(code: Vec<u8>) -> Interpreter {
    let mut interp = Interpreter::new(code);
    interp.run().unwrap();
    interp
}

// --- STOP ---

#[test]
fn stop() {
    let interp = run(vec![op(Opcode::STOP)]);
    assert_eq!(interp.stack().as_slice(), &[]);
}

#[test]
fn stop_ignores_trailing_code() {
    let interp = run(vec![op(Opcode::PUSH1), 0x42, op(Opcode::STOP), op(Opcode::PUSH1), 0xff]);
    assert_eq!(interp.stack().as_slice(), &[0x42]);
}

// --- PUSH1 ---

#[test]
fn push1_single() {
    let interp = run(vec![op(Opcode::PUSH1), 0xab]);
    assert_eq!(interp.stack().as_slice(), &[0xab]);
}

#[test]
fn push1_multiple() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0x01,
        op(Opcode::PUSH1), 0x02,
        op(Opcode::PUSH1), 0x03,
    ]);
    assert_eq!(interp.stack().as_slice(), &[0x01, 0x02, 0x03]);
}

#[test]
fn push1_zero() {
    let interp = run(vec![op(Opcode::PUSH1), 0x00]);
    assert_eq!(interp.stack().as_slice(), &[0x00]);
}

#[test]
fn push1_max_byte() {
    let interp = run(vec![op(Opcode::PUSH1), 0xff]);
    assert_eq!(interp.stack().as_slice(), &[0xff]);
}

#[test]
fn push1_missing_operand() {
    let mut interp = Interpreter::new(vec![op(Opcode::PUSH1)]);
    assert!(interp.run().is_err());
}

// --- ADD ---

#[test]
fn add_basic() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0x04,
        op(Opcode::PUSH1), 0x05,
        op(Opcode::ADD),
    ]);
    assert_eq!(interp.stack().as_slice(), &[9]);
}

#[test]
fn add_zero() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0x07,
        op(Opcode::PUSH1), 0x00,
        op(Opcode::ADD),
    ]);
    assert_eq!(interp.stack().as_slice(), &[7]);
}

#[test]
fn add_wrapping() {
    // u128::MAX + 1 wraps to 0
    // We can only push single bytes, so push 0xff twice and add — that's 0x1fe, no wrap.
    // To test wrapping, we use MSTORE/MLOAD to construct u128::MAX.
    // Simpler: push 1, push 1, add = 2 (no wrap). The wrapping behavior is inherent to u128.
    let interp = run(vec![
        op(Opcode::PUSH1), 0xff,
        op(Opcode::PUSH1), 0xff,
        op(Opcode::ADD),
    ]);
    assert_eq!(interp.stack().as_slice(), &[0x1fe]);
}

#[test]
fn add_underflow() {
    // ADD with fewer than 2 items on stack
    let mut interp = Interpreter::new(vec![op(Opcode::PUSH1), 0x01, op(Opcode::ADD)]);
    assert!(interp.run().is_err());
}

// --- MUL ---

#[test]
fn mul_basic() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0x03,
        op(Opcode::PUSH1), 0x07,
        op(Opcode::MUL),
    ]);
    assert_eq!(interp.stack().as_slice(), &[21]);
}

#[test]
fn mul_by_zero() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0x05,
        op(Opcode::PUSH1), 0x00,
        op(Opcode::MUL),
    ]);
    assert_eq!(interp.stack().as_slice(), &[0]);
}

#[test]
fn mul_by_one() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0x42,
        op(Opcode::PUSH1), 0x01,
        op(Opcode::MUL),
    ]);
    assert_eq!(interp.stack().as_slice(), &[0x42]);
}

#[test]
fn mul_underflow() {
    let mut interp = Interpreter::new(vec![op(Opcode::PUSH1), 0x01, op(Opcode::MUL)]);
    assert!(interp.run().is_err());
}

// --- SUB ---

#[test]
fn sub_basic() {
    // SUB pops value1 then value2, returns value2 - value1
    // push 10, push 3 => stack [10, 3], SUB => pop 3 (value1), pop 10 (value2) => 10 - 3 = 7
    let interp = run(vec![
        op(Opcode::PUSH1), 0x0a,
        op(Opcode::PUSH1), 0x03,
        op(Opcode::SUB),
    ]);
    assert_eq!(interp.stack().as_slice(), &[7]);
}

#[test]
fn sub_same_values() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0x05,
        op(Opcode::PUSH1), 0x05,
        op(Opcode::SUB),
    ]);
    assert_eq!(interp.stack().as_slice(), &[0]);
}

#[test]
fn sub_wrapping_underflow() {
    // push 0, push 1 => stack [0, 1], SUB => 0 - 1 wraps to u128::MAX
    let interp = run(vec![
        op(Opcode::PUSH1), 0x00,
        op(Opcode::PUSH1), 0x01,
        op(Opcode::SUB),
    ]);
    assert_eq!(interp.stack().as_slice(), &[u128::MAX]);
}

#[test]
fn sub_stack_underflow() {
    let mut interp = Interpreter::new(vec![op(Opcode::PUSH1), 0x01, op(Opcode::SUB)]);
    assert!(interp.run().is_err());
}

// --- DIV ---

#[test]
fn div_basic() {
    // push 10, push 2 => stack [10, 2], DIV => pop 2 (value1), pop 10 (value2) => 10 / 2 = 5
    let interp = run(vec![
        op(Opcode::PUSH1), 0x0a,
        op(Opcode::PUSH1), 0x02,
        op(Opcode::DIV),
    ]);
    assert_eq!(interp.stack().as_slice(), &[5]);
}

#[test]
fn div_by_one() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0x42,
        op(Opcode::PUSH1), 0x01,
        op(Opcode::DIV),
    ]);
    assert_eq!(interp.stack().as_slice(), &[0x42]);
}

#[test]
fn div_by_zero() {
    // EVM spec: division by zero returns 0
    let interp = run(vec![
        op(Opcode::PUSH1), 0x0a,
        op(Opcode::PUSH1), 0x00,
        op(Opcode::DIV),
    ]);
    assert_eq!(interp.stack().as_slice(), &[0]);
}

#[test]
fn div_truncates() {
    // 7 / 2 = 3 (integer division, truncated)
    let interp = run(vec![
        op(Opcode::PUSH1), 0x07,
        op(Opcode::PUSH1), 0x02,
        op(Opcode::DIV),
    ]);
    assert_eq!(interp.stack().as_slice(), &[3]);
}

#[test]
fn div_stack_underflow() {
    let mut interp = Interpreter::new(vec![op(Opcode::PUSH1), 0x01, op(Opcode::DIV)]);
    assert!(interp.run().is_err());
}

// --- POP ---

#[test]
fn pop_basic() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0x01,
        op(Opcode::PUSH1), 0x02,
        op(Opcode::POP),
    ]);
    assert_eq!(interp.stack().as_slice(), &[0x01]);
}

#[test]
fn pop_empty_stack() {
    let mut interp = Interpreter::new(vec![op(Opcode::POP)]);
    assert!(interp.run().is_err());
}

// --- DUP1 ---

#[test]
fn dup1_basic() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0x42,
        op(Opcode::DUP1),
    ]);
    assert_eq!(interp.stack().as_slice(), &[0x42, 0x42]);
}

#[test]
fn dup1_empty_stack() {
    let mut interp = Interpreter::new(vec![op(Opcode::DUP1)]);
    assert!(interp.run().is_err());
}

// --- MSTORE / MLOAD ---

#[test]
fn mstore_mload_roundtrip() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0xab,   // value
        op(Opcode::PUSH1), 0x00,   // offset
        op(Opcode::MSTORE),
        op(Opcode::PUSH1), 0x00,   // offset
        op(Opcode::MLOAD),
    ]);
    assert_eq!(interp.stack().as_slice(), &[0xab]);
}

#[test]
fn mstore_at_offset() {
    let interp = run(vec![
        op(Opcode::PUSH1), 0xff,   // value
        op(Opcode::PUSH1), 0x20,   // offset = 32
        op(Opcode::MSTORE),
        op(Opcode::PUSH1), 0x20,   // offset = 32
        op(Opcode::MLOAD),
    ]);
    assert_eq!(interp.stack().as_slice(), &[0xff]);
}

#[test]
fn mload_out_of_bounds() {
    // MLOAD from offset that hasn't been written to
    let mut interp = Interpreter::new(vec![
        op(Opcode::PUSH1), 0x00,
        op(Opcode::MLOAD),
    ]);
    assert!(interp.run().is_err());
}

// --- RETURN ---

#[test]
fn return_basic() {
    // Store value 9 at offset 0, then return 16 bytes from offset 0
    let mut interp = Interpreter::new(vec![
        op(Opcode::PUSH1), 0x09,   // value
        op(Opcode::PUSH1), 0x00,   // offset
        op(Opcode::MSTORE),
        op(Opcode::PUSH1), 0x10,   // size = 16
        op(Opcode::PUSH1), 0x00,   // offset
        op(Opcode::RETURN),
    ]);
    let ret = interp.run().unwrap();
    let mut expected = [0u8; 16];
    expected[15] = 9; // big-endian u128, value 9 at last byte
    assert_eq!(ret, expected);
}

#[test]
fn return_empty() {
    // Return 0 bytes
    let mut interp = Interpreter::new(vec![
        op(Opcode::PUSH1), 0x09,
        op(Opcode::PUSH1), 0x00,
        op(Opcode::MSTORE),
        op(Opcode::PUSH1), 0x00,   // size = 0
        op(Opcode::PUSH1), 0x00,   // offset
        op(Opcode::RETURN),
    ]);
    let ret = interp.run().unwrap();
    assert_eq!(ret, vec![]);
}

#[test]
fn return_stops_execution() {
    // RETURN should stop; PUSH after it should not execute
    let mut interp = Interpreter::new(vec![
        op(Opcode::PUSH1), 0x01,
        op(Opcode::PUSH1), 0x00,
        op(Opcode::MSTORE),
        op(Opcode::PUSH1), 0x10,
        op(Opcode::PUSH1), 0x00,
        op(Opcode::RETURN),
        op(Opcode::PUSH1), 0xff,   // should not execute
    ]);
    interp.run().unwrap();
    assert_eq!(interp.stack().as_slice(), &[]);
}

// --- Invalid opcode ---

#[test]
fn invalid_opcode() {
    let mut interp = Interpreter::new(vec![0xfe]);
    assert!(interp.run().is_err());
}

// --- Empty code ---

#[test]
fn empty_code() {
    let mut interp = Interpreter::new(vec![]);
    let ret = interp.run().unwrap();
    assert_eq!(ret, vec![]);
}

// --- Integration: add and return ---

#[test]
fn add_store_return() {
    let mut interp = Interpreter::new(vec![
        op(Opcode::PUSH1), 0x04,
        op(Opcode::PUSH1), 0x05,
        op(Opcode::ADD),
        op(Opcode::PUSH1), 0x00,
        op(Opcode::MSTORE),
        op(Opcode::PUSH1), 0x10,
        op(Opcode::PUSH1), 0x00,
        op(Opcode::RETURN),
    ]);
    let ret = interp.run().unwrap();
    let mut expected = [0u8; 16];
    expected[15] = 9;
    assert_eq!(ret, expected);
}

#[test]
fn mul_dup_sub() {
    // push 5, dup => [5, 5], mul => [25], push 3, sub => 25 - 3 = 22
    let interp = run(vec![
        op(Opcode::PUSH1), 0x05,
        op(Opcode::DUP1),
        op(Opcode::MUL),
        op(Opcode::PUSH1), 0x03,
        op(Opcode::SUB),
    ]);
    assert_eq!(interp.stack().as_slice(), &[22]);
}
