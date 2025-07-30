use crate::VM;

#[test]
fn dvd() {
    let mut vm = VM::new();
    vm.load_program("programs/dvd.as").unwrap();
    for i in 1..=2000 {
        if i % 500 == 0 {
            vm.io_devices.screen.display();
        }
        vm.clock();
    }
}
