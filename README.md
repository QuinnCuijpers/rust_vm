# Rust VM

This project implements a simple virtual machine in Rust, inspired by [mattbattwings' Minecraft Redstone Computer](https://www.youtube.com/watch?v=osFa7nwHHz4&list=PL5LiOvrbVo8nPTtdXAdSmDWzu85zzdgRT).

## Arithmetic Logic Unit (ALU)

Performs operations on 8-bit values using a custom `Bits` struct that represents bits as an array of booleans. The ALU is designed as a **carry cancel adder (cca)**.

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

---

## Register
A simple registerbank implementation using 16 simulated dual-read registers. The register supports basic operations such as reading and writing values.


# License
This project is licensed under the MIT License. See the [license](license.txt) file for details.