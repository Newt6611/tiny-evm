# tiny-evm

A minimal Ethereum Virtual Machine implementation for learning purposes. Built to understand how the EVM works at a low level.

This is a practice project and does not include gas calculation.

## Supported Opcodes

| Opcode | Hex | Description |
|--------|-----|-------------|
| STOP | 0x00 | Halt execution |
| ADD | 0x01 | Addition |
| MUL | 0x02 | Multiplication |
| SUB | 0x03 | Subtraction |
| DIV | 0x04 | Integer division (div by zero returns 0) |
| POP | 0x50 | Remove top stack item |
| MLOAD | 0x51 | Load word from memory |
| MSTORE | 0x52 | Store word to memory |
| PUSH1 | 0x60 | Push 1-byte value |
| DUP1 | 0x80 | Duplicate top stack item |
| RETURN | 0xf3 | Return data from memory |

## Run

```sh
cargo run
```

## Test

```sh
cargo test
```
