#![allow(unused_results)]
use rust_vm::registers::register_file::RegisterBank;
use rust_vm::registers::RegisterFile;
use rust_vm::Bits;
use rust_vm::VM;

#[test]
fn vm_execute_program() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/test.as").unwrap();
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
    vm.execute_program("tests/test_programs/test.as").unwrap();
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
    vm.execute_program("tests/test_programs/test2.as").unwrap();
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
    vm.execute_program("tests/test_programs/test3.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 4);
    assert_eq!(vm.reg_file.register_banks[0][2].to_usize(), 8);
}

#[test]
fn fib() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/fib.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 1);
    assert_eq!(vm.reg_file.register_banks[0][2].to_usize(), 1);
    assert_eq!(vm.reg_file.register_banks[0][3].to_usize(), 2);
    assert_eq!(vm.reg_file.register_banks[0][4].to_usize(), 3);
    assert_eq!(vm.reg_file.register_banks[0][5].to_usize(), 5);
}

#[test]
fn vm_program_4() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/test4.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 3);
}

#[test]
fn vm_program_5() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/test5.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 3);
}

#[test]
fn vm_program_6() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/test6.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 1);
}

#[test]
fn vm_program_7() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/test7.as").unwrap();
}

#[test]
fn vm_program_8() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/test8.as").unwrap();
}

#[test]
fn vm_fib_2() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/fib2.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][3].to_usize(), 13);
}

#[test]
fn vm_program_9() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/test9.as").unwrap();
    assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 3);
}

#[test]
fn vm_program_bubble_sort() {
    let mut vm = VM::new();
    let arr: Vec<u64> = vec![6, 2, 5, 8, 9, 1, 3, 4, 7, 0];
    let mut bit_arr = [Bits::<8>::from(0u8); 256];
    for (i, &val) in arr.iter().enumerate() {
        bit_arr[i] = Bits::from(val).resize();
    }
    vm.data_memory.memory[..arr.len()].copy_from_slice(&bit_arr[..arr.len()]);
    vm.data_memory.memory[0..8]
        .iter()
        .for_each(|b| println!("{:?}", b.to_usize()));
    vm.execute_program("tests/test_programs/test_bubble_sort.as")
        .unwrap();
    let res = vm.data_memory.memory[..arr.len()]
        .iter()
        .map(|bits| bits.to_usize())
        .collect::<Vec<_>>();
    println!("Sorted array: {res:?}");
    assert!(res.is_sorted());
}

#[test]
fn vm_program_number_display() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/test_number_display1.as")
        .unwrap();
    assert_eq!(vm.io_devices.number_display.display(), "200");

    vm.execute_program("tests/test_programs/test_number_display2.as")
        .unwrap();
    assert_eq!(vm.io_devices.number_display.display(), "201");

    vm.execute_program("tests/test_programs/test_number_display3.as")
        .unwrap();
    assert_eq!(vm.io_devices.number_display.display(), "-55");

    vm.execute_program("tests/test_programs/test_number_display4.as")
        .unwrap();
    assert_eq!(vm.io_devices.number_display.display(), "201");

    vm.execute_program("tests/test_programs/test_number_display5.as")
        .unwrap();
    assert_eq!(vm.io_devices.number_display.display(), "0");

    vm.execute_program("tests/test_programs/test_number_display6.as")
        .unwrap();
    assert_eq!(vm.io_devices.number_display.display(), "202");
}

#[test]
fn vm_program_character_display() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/hello.as").unwrap();
    assert_eq!(
        vm.io_devices.character_display.active,
        "Hello!".to_ascii_lowercase()
    );
}

#[test]
fn vm_program_screen() {
    let mut vm = VM::default();
    vm.execute_program("tests/test_programs/screen.as").unwrap();
    for row in vm.io_devices.screen.active.iter() {
        for &pixel in row.iter() {
            assert!(!pixel, "Expected all pixels to be false");
        }
    }
}
