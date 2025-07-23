use crate::VM;
use std::fs;
mod parse {
    use super::*;
    #[test]
    fn parse_2048() {
        let mut vm = VM::default();
        let res = vm.load_program("programs/2048.as");
        res.unwrap();
    }

    #[test]
    fn parse_calc() {
        let mut vm = VM::default();
        let res = vm.load_program("programs/calculator.as");
        res.unwrap();
    }

    #[test]
    fn parse_connect4() {
        let mut vm = VM::default();
        let res = vm.load_program("programs/connect4.as");
        res.unwrap();
    }

    #[test]
    fn parse_dvd() {
        let mut vm = VM::default();
        let res = vm.load_program("programs/dvd.as");
        res.unwrap();
    }

    #[test]
    fn parse_gol() {
        let mut vm = VM::default();
        let res = vm.load_program("programs/gol.as");
        res.unwrap();
    }

    #[test]
    fn parse_helloworld() {
        let mut vm = VM::default();
        let res = vm.load_program("programs/helloworld.as");
        res.unwrap();
    }

    #[test]
    fn parse_maze() {
        let mut vm = VM::default();
        let res = vm.load_program("programs/maze.as");
        res.unwrap();
    }

    #[test]
    fn parse_minesweeper() {
        let mut vm = VM::default();
        let res = vm.load_program("programs/minesweeper.as");
        res.unwrap();
    }
}

fn assert_mc_equivalent(program_name: &str) {
    let generated_path = format!("programs/{program_name}.mc");
    let check_path = format!("check_mc/{program_name}.mc");

    let generated =
        fs::read(&generated_path).unwrap_or_else(|_| panic!("Failed to read {generated_path}"));
    let expected = fs::read(&check_path).unwrap_or_else(|_| panic!("Failed to read {check_path}"));

    if generated != expected {
        let generated_lines: Vec<_> = generated.split(|&b| b == b'\n').collect();
        let expected_lines: Vec<_> = expected.split(|&b| b == b'\n').collect();
        let min_len = generated_lines.len().min(expected_lines.len());
        for i in 0..min_len {
            if generated_lines[i] != expected_lines[i] {
                let gen_line = String::from_utf8_lossy(generated_lines[i]);
                let exp_line = String::from_utf8_lossy(expected_lines[i]);
                panic!(
                    "Machine code for {program_name} does not match reference at line {}:\n  generated: {}\n  expected: {}",
                    i + 1,
                    gen_line,
                    exp_line
                );
            }
        }
        if generated_lines.len() != expected_lines.len() {
            panic!(
                "Machine code for {program_name} has different number of lines: generated={}, expected={}",
                generated_lines.len(),
                expected_lines.len()
            );
        }
        panic!(
            "Machine code for {program_name} does not match reference, but no differing line found"
        );
    }
}

#[test]
fn mc_equivalence_2048() {
    assert_mc_equivalent("2048");
}

#[test]
fn mc_equivalence_calculator() {
    assert_mc_equivalent("calculator");
}

#[test]
fn mc_equivalence_connect4() {
    assert_mc_equivalent("connect4");
}

#[test]
fn mc_equivalence_dvd() {
    assert_mc_equivalent("dvd");
}

#[test]
fn mc_equivalence_gol() {
    assert_mc_equivalent("gol");
}

#[test]
fn mc_equivalence_helloworld() {
    assert_mc_equivalent("helloworld");
}

#[test]
fn mc_equivalence_maze() {
    assert_mc_equivalent("maze");
}

#[test]
fn mc_equivalence_minesweeper() {
    assert_mc_equivalent("minesweeper");
}
