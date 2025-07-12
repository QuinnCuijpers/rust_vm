# Rust VM

This project implements a simple virtual machine in Rust, inspired by [mattbattwings' Minecraft Redstone Computer](https://www.youtube.com/watch?v=osFa7nwHHz4&list=PL5LiOvrbVo8nPTtdXAdSmDWzu85zzdgRT).

## Architecture

- **ALU (Arithmetic Logic Unit):**  
    Performs operations on 8-bit values using a custom `Bits` struct.

### Supported ALU Operations

| Operation      | Description         |
|----------------|--------------------|
| Addition       | `a + b`            |
| Subtraction    | `a - b`            |
| Bitwise AND    | `a & b`            |
| Bitwise OR     | `a \| b`           |
| Bitwise XOR    | `a ^ b`            |
| Bitwise NOT    | `!a`               |
| Bitwise XNOR   | `~(a ^ b)`         |
| Bitwise NAND   | `~(a & b)`         |
| Bitwise NOR    | `~(a \| b)`        |

> The ALU is designed as a **bitwise carry cancel adder**.

---