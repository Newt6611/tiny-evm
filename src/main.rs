use tiny_evm::{
    interpreter::Interpreter,
    opcodes::{op, Opcode},
};

fn main() {
    let code = vec![
        op(Opcode::PUSH1),
        0x04,
        op(Opcode::PUSH1),
        0x05,
        op(Opcode::ADD),

        op(Opcode::PUSH1),
        0x00,
        op(Opcode::MSTORE),
        op(Opcode::PUSH1),
        0x10,
        op(Opcode::PUSH1),
        0x00,
        op(Opcode::RETURN),
    ];
    let mut inter = Interpreter::new(code);

    let ret = inter.run().unwrap();
    println!("return: {:?}", ret);
    println!("stack: {:?}", inter.stack().as_slice());
    println!("memory: {:?}", inter.memory().as_slice());
}
