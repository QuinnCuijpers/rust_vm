# Rust VM

This project implements a simple virtual machine in Rust, inspired by [mattbattwings' Minecraft Redstone Computer](https://www.youtube.com/watch?v=osFa7nwHHz4&list=PL5LiOvrbVo8nPTtdXAdSmDWzu85zzdgRT).

## Features

- **Parser:**  
  Parses assembly-like instructions from files into .mc files, with robust error handling for missing operands, invalid instructions, and file errors.

- **Arithmetic Logic Unit (ALU):**  
  Performs operations on 8-bit values using a custom `Bits` struct (array of booleans).  
  Implements a carry cancel adder (CCA) and supports multiple bitwise and arithmetic operations.

- **Register Bank:**  
  Simulated dual-read register bank with 16 registers per bank, supporting read, write, enable/disable, and validation logic.

- **Instruction Memory:**  
  Stores up to 1024 instructions, each instruction is a `[Bits<4>; 4]` array.

- **Control ROM:**  
  Decodes opcodes into control signals for the VM.

- **Testing:**  
  Comprehensive unit tests for ALU, bits, parser, register, and instruction memory modules.  
  Tests are organized by feature and error type. Including code coverage reports.

## Supported Assembly Instructions and Syntax

The VM supports a simple assembly-like language. Each instruction consists of an opcode followed by its operands (registers or immediate values).  
Below are the supported instructions and their syntax:

| Instruction | Syntax Example         | Description                                      |
|-------------|-----------------------|--------------------------------------------------|
| `LDI`       | `LDI r1 42`           | Load immediate value `42` into register `r1`     |
| `ADD`       | `ADD r1 r2 r3`        | Add `r1` and `r2`, store result in `r3`          |
| `SUB`       | `SUB r1 r2 r3`        | Subtract `r2` from `r1`, store result in `r3`    |
| `AND`       | `AND r1 r2 r3`        | Bitwise AND of `r1` and `r2`, store in `r3`      |
| `NOR`       | `NOR r1 r2 r3`        | Bitwise NOR of `r1` and `r2`, store in `r3`      |
| `XOR`       | `XOR r1 r2 r3`        | Bitwise XOR of `r1` and `r2`, store in `r3`      |
| `RSH`       | `RSH r1 r2`           | Right shift `r1` by 1, store result in `r2`      |
| `NOP`       | `NOP`                 | No operation                                     |

**Notes:**
- Registers are specified as `r0`, `r1`, ..., `r15`.
- Immediate values (for `LDI`) must fit in 8 bits.
- All instructions and operands are space-separated.
- Extra or missing operands will result in a parse error.

**Example:**
```
LDI r1 1
LDI r2 1
ADD r1 r2 r3
RSH
```

## Usage

1. Write your program in assembly-like syntax.
2. Run the VM using the provided API in `lib.rs`.
3. Inspect register and memory state using the display function of the register file.

### Example: Running a Program from `fib.as`

```rust
use rust_vm::VM;

fn main() {
    let mut vm = VM::default();
    vm.execute_program("fib.as");
    vm.reg_file.display(); // Prints register banks
}
```

### Example Assembly (`fib.as`)

```
LDI r1 1
LDI r2 1
ADD r1 r2 r3
ADD r2 r3 r4
ADD r3 r4 r5
```

This example computes the first few Fibonacci numbers and stores them in registers.  
You can modify `fib.as` to try your own programs!

## License

This project is licensed under the MIT License. See the [license](license.txt) file for details.