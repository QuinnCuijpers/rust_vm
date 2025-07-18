# Rust VM

This project implements a simple virtual machine in Rust, inspired by [mattbattwings' Minecraft Redstone Computer](https://www.youtube.com/watch?v=osFa7nwHHz4&list=PL5LiOvrbVo8nPTtdXAdSmDWzu85zzdgRT).

## Features

- **Parser:**  
  Parses assembly-like instructions, supports labels, and provides robust error handling for missing operands, invalid instructions, undefined labels, and file errors.

- **Arithmetic Logic Unit (ALU):**  
  Performs operations on 8-bit values using a custom `Bits` struct (array of booleans).  
  Implements a carry cancel adder (CCA) and supports multiple bitwise and arithmetic operations.  
  Sets flags (zero, carry) after each operation, which are used for conditional branching.

- **Register Bank:**  
  Simulated dual-read register bank with 16 registers per bank, supporting read, write, enable/disable, and validation logic.

- **Instruction Memory:**  
  Stores up to 1024 instructions, each instruction is a `[Bits<4>; 4]` array.

- **Control ROM:**  
  Decodes opcodes into control signals for the VM.

- **Program Counter (PC):**  
  Tracks the address of the current instruction being executed.  
  Automatically increments after each instruction, unless modified by control logic (e.g halts).

- **Testing:**  
  Comprehensive unit tests for ALU, bits, parser, register, and instruction memory modules.  
  Tests are organized by feature and error type.  
  Code coverage is measured using [`cargo llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov).

## Supported Assembly Instructions and Syntax

The VM supports a simple assembly-like language. Each instruction consists of an opcode followed by its operands (registers or immediate values). For a complete list of opcodes and their binary representations, see [mattbattwings' Minecraft Redstone Computer playlist](https://www.youtube.com/watch?v=osFa7nwHHz4&list=PL5LiOvrbVo8nPTtdXAdSmDWzu85zzdgRT). The local opcode file [opcodes.xlsx](opcodes.xlsx) reflects only the instructions currently implemented in this VM.
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
| `HLT`       | `HLT`                 | Halt execution                                   |
| `BRH`       | `BRH cond addr`        | Branch to `addr` if condition `cond` is met      |
| `JMP`       | `JMP addr`            | Jump to instruction at `addr`                    |
| `CAL`       | `CAL addr`            | Call subroutine at `addr` (pushes return address)|
| `RET`       | `RET`                 | Return from subroutine (pops return address)     |

**Pseudocode Instructions:**

Some instructions are provided as convenient pseudocode and are automatically expanded to real instructions during parsing:
| Pseudocode   | Expansion         | Description                        |
|-------------|-------------------|------------------------------------|
| `INC rX`    | `ADI rX 1`        | Increment register `rX` by 1       |
| `DEC rX`    | `ADI rX 255`      | Decrement register `rX` by 1       |
| `CMP rX rY` | `SUB rX rY r0`    | Compare `rX` and `rY`              |

**BRH Supported Conditions:**

| Condition | Syntax      | Meaning                       |
|-----------|------------|-------------------------------|
| zero      | `=` `eq` `z` `zero`      | Branch if result is zero         |
| notzero   | `!=` `ne` `nz` `notzero` | Branch if result is not zero     |
| carry     | `>=` `ge` `c` `carry`    | Branch if carry flag is set      |
| notcarry  | `<` `lt` `nc` `notcarry` | Branch if carry flag is not set  |


- **Labels:**  
  You can define labels in your assembly code using `.label`. Branch/Jump instructions (`BRH`/`JMP`) can use labels as targets.

-

**Notes:**
- Registers are specified as `r0`, `r1`, ..., `r15`.
- Register 0 is a zero register (always reads as 0 and writes are ignored).
- Immediate values (for `LDI`) must fit in 8 bits.
- All instructions and operands are space-separated.
- Extra or missing operands will result in a parse error.
- `CAL` and `RET` provide basic subroutine call/return support.
- The call stack has a maximum depth of 16; deeper recursion will cause overflow errors.

**Example:**
```
    CAL .add3
    HLT
.add3 CAL .add1
    CAL .add2
    RET
.add1 ADI r1 1
    RET
.add2 ADI r1 2
    RET
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
.fib LDI r1 8 
    LDI r2 0
    LDI r3 1
    LDI r4 0
.loop DEC r1
    BRH nc .done
    ADD r3 r0 r2
    ADD r4 r0 r3
    ADD r2 r3 r4
    JMP .loop
.done HLT
```

This example computes the 8th Fibonacci number and stores them in the 4th register.  
You can modify `fib.as` to try your own programs!

## Testing & Coverage

Run all tests:
```sh
cargo test
```

Generate a coverage report using `cargo llvm-cov` that opens in your browser:
```sh
cargo llvm-cov --open
```

### Code Coverage in VS Code Gutters

To view code coverage directly in VS Code gutters, install the [Coverage Gutters](https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters) extension.  
After generating a coverage report (e.g., `lcov.info`), open your project in VS Code and use Coverage Gutters to visualize coverage in the editor.

```sh
cargo llvm-cov --lcov --output-path lcov.info
```

## License

This project is licensed under the MIT License. See the [license](license.txt) file for details.