# Rust VM

This project implements a simple virtual machine in Rust, inspired by [mattbattwings' Minecraft Redstone Computer](https://www.youtube.com/watch?v=osFa7nwHHz4&list=PL5LiOvrbVo8nPTtdXAdSmDWzu85zzdgRT).

**Original GitHub Repository:**
[https://github.com/mattbatwings/BatPU-2](https://github.com/mattbatwings/BatPU-2)

## Features & Architecture


## IO Devices

The VM supports several IO devices for interacting with programs and visualizing output:

- **Character Display:**
  - Allows output of ASCII characters to a virtual display.
  - Useful for printing text or debugging output from your program.
  - Accessed via memory-mapped IO addresses (see code for details).

- **Number Display:**
  - Displays numeric values (e.g., register or memory contents) in decimal or hexadecimal.
  - Useful for debugging or showing program results.

- **Screen Device:**
  - Provides a pixel-based display for graphical output.
  - Programs can write to the screen buffer to draw simple graphics or animations.

- **Random Number Generator (RNG):**
  - Provides random values for use in games or randomized algorithms.
  - Accessed via a special memory-mapped address.

To use these devices, write to or read from their designated memory-mapped addresses in your assembly program. See the `io_devices/` module and example programs in `programs/` for usage patterns.

## Components


- **Parser:**
  - Parses assembly-like instructions with support for labels, definitions, and robust error handling (missing/extra operands, invalid instructions, undefined labels, file errors).
  - Pseudoinstructions (e.g., `INC`, `DEC`, `CMP`) are expanded automatically.


- **Arithmetic Logic Unit (ALU):**
  - Operates on 8-bit values using a custom `Bits` struct (array of booleans).
  - Implements a carry-cancel adder (CCA) and supports bitwise/arithmetic ops.
  - Sets flags (zero, carry) for conditional branching.


- **Register Bank:**
  - Dual-read register bank with 16 registers, supporting read/write, enable/disable, and validation logic.


- **Instruction Memory:**
  - Stores up to 1024 instructions, each as a `Bits<16>`.

- **Control ROM:**
  - Decodes opcodes into control signals for the VM.


- **Program Counter (PC):**
  - Tracks the address of the current instruction.
  - Increments after each instruction unless modified by control logic (e.g., halt, jump, call/return).

## Testing  
  - Comprehensive unit tests for all major modules (ALU, bits, parser, register, instruction memory, VM execution).
  - Tests are organized by feature and error type.
  - Code coverage is measured using [`cargo llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov).


## Supported Assembly Instructions & Syntax


The VM supports a simple assembly-like language, where each instruction consists of an opcode followed by its operands (registers, immediate values, or memory addresses). Instructions are space-separated and support labels for control flow. For a detailed list of supported opcodes, their syntax, and binary layouts, refer to the tables below or consult the [BatPU-2 ISA documentation](BatPU-2%20ISA.xlsx) and [mattbattwings' playlist](https://www.youtube.com/watch?v=osFa7nwHHz4&list=PL5LiOvrbVo8nPTtdXAdSmDWzu85zzdgRT).

The assembler provides robust error handling for invalid instructions, missing or extra operands, and undefined labels. Pseudoinstructions (such as `INC`, `DEC`, `CMP`, etc.) are automatically expanded to real instructions during parsing, making it easier to write concise and readable assembly code.

See the tables below for a summary of supported instructions, pseudoinstructions, and branching conditions.

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
| `LOD`       | `LOD r1 r2 [off]`      | Load value from data memory at address in `r2` plus optional 4-bit offset `off` into `r1` |
| `STR`       | `STR r1 r2 [off]`      | Store value from `r1` into data memory at address in `r2` plus optional 4-bit offset `off` |


**Pseudoinstructions:**

Some instructions are provided as convenient pseudocode and are automatically expanded to real instructions during parsing:

| Pseudocode      | Expansion         | Description                        |
|----------------|-------------------|------------------------------------|
| `CMP A B`      | `SUB A B r0`      | Compare `A` and `B`                |
| `MOV A C`      | `ADD A r0 C`      | Move value from `A` to `C`         |
| `LSH A C`      | `ADD A A C`       | Left shift `A` by 1, store in `C`  |
| `INC A`        | `ADI A 1`         | Increment register `A` by 1        |
| `DEC A`        | `ADI A 255`       | Decrement register `A` by 1        |
| `NOT A C`      | `NOR A r0 C`      | Bitwise NOT of `A`, store in `C`   |
| `NEG A C`      | `SUB r0 A C`      | Negate `A`, store in `C`           |


**BRH Supported Conditions:**

| Condition | Syntax      | Meaning                       |
|-----------|------------|-------------------------------|
| zero      | `=` `eq` `z` `zero`      | Branch if result is zero         |
| notzero   | `!=` `ne` `nz` `notzero` | Branch if result is not zero     |
| carry     | `>=` `ge` `c` `carry`    | Branch if carry flag is set      |
| notcarry  | `<` `lt` `nc` `notcarry` | Branch if carry flag is not set  |



- **Labels:**
  - Define labels in your assembly code using `.label` (either on their own line or inline with an instruction).
  - Branch/Jump/Call instructions (`BRH`/`JMP`/`CAL`) can use labels as targets.
  - Labels above comments or blank lines are resolved to the next instruction.

-


**Notes:**
- Registers: `r0`–`r15` (with `r0` as a zero register: always reads as 0, writes are ignored).
- Immediate values (for `LDI`) must fit in 8 bits.
- All instructions and operands are space-separated.
- Extra or missing operands will result in a parse error.
- `CAL` and `RET` provide basic subroutine call/return support (call stack max depth: 16).



## Usage


1. Write your program in assembly-like syntax (see examples in `programs/`).
2. Run the VM using the provided API in `lib.rs`.
3. Inspect register and memory state using the display function of the register file, or visualize output using the screen device.

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