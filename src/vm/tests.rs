#![allow(unused_results)]
use crate::register::{RegisterBank, RegisterFile};
use crate::{bits::Bits, VM};

#[test]
fn vm_execute_program() {
    let mut vm = VM::default();
    vm.execute_program("test.as").unwrap();
    vm.reg_file.display();
}

#[test]
fn vm_execution() {
    let mut arr = [Bits::from(0u8); 16];
    arr[1] = Bits::from(7u8);
    arr[2] = Bits::from(8u8);
    arr[3] = Bits::from(6u8);
    let mut vm = VM::new();
    vm.reg_file = RegisterFile::new(RegisterBank::from(arr));
    vm.execute_program("test.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 7,);
    assert_eq!(vm.reg_file.register_banks[0][2].to_usize(), 7,);
    assert_eq!(vm.reg_file.register_banks[0][3].to_usize(), 9,);
    assert_eq!(vm.reg_file.register_banks[1][1].to_usize(), 7,);
    assert_eq!(vm.reg_file.register_banks[1][2].to_usize(), 7,);
    assert_eq!(vm.reg_file.register_banks[1][3].to_usize(), 9,);
}

#[test]
fn nop() {
    std::fs::write("nop.as", "NOP\n").unwrap();
    std::fs::write("nop.as", "HLT").unwrap();
    let mut vm = VM::default();
    vm.execute_program("nop.as").unwrap();
    std::fs::remove_file("nop.as").unwrap();
    std::fs::remove_file("nop.mc").unwrap();
}

#[test]
fn vm_execute_program_2() {
    let mut arr = [Bits::from(0u8); 16];
    arr[1] = Bits::from(1u8);
    arr[2] = Bits::from(1u8);
    let mut vm = VM::new();
    vm.reg_file = RegisterFile::new(RegisterBank::from(arr));
    vm.execute_program("test2.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 3);
    assert_eq!(vm.reg_file.register_banks[0][2].to_usize(), 1);
    assert_eq!(vm.reg_file.register_banks[0][3].to_usize(), 2);
    assert_eq!(vm.reg_file.register_banks[0][4].to_usize(), 3);
    assert_eq!(vm.reg_file.register_banks[1][1].to_usize(), 3);
    assert_eq!(vm.reg_file.register_banks[1][2].to_usize(), 1);
    assert_eq!(vm.reg_file.register_banks[1][3].to_usize(), 2);
    assert_eq!(vm.reg_file.register_banks[1][4].to_usize(), 3);
}

#[test]
fn vm_program_3() {
    let mut vm = VM::default();
    vm.execute_program("test3.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 4);
    assert_eq!(vm.reg_file.register_banks[0][2].to_usize(), 8);
}

#[test]
fn fib() {
    let mut vm = VM::default();
    vm.execute_program("fib.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 1);
    assert_eq!(vm.reg_file.register_banks[0][2].to_usize(), 1);
    assert_eq!(vm.reg_file.register_banks[0][3].to_usize(), 2);
    assert_eq!(vm.reg_file.register_banks[0][4].to_usize(), 3);
    assert_eq!(vm.reg_file.register_banks[0][5].to_usize(), 5);
}

#[test]
fn vm_program_4() {
    let mut vm = VM::default();
    vm.execute_program("test4.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 3);
}

#[test]
fn vm_program_5() {
    let mut vm = VM::default();
    vm.execute_program("test5.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 3);
}
