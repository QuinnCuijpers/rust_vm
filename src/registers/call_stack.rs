use crate::registers::Register;
use crate::Address;
// TODO: refactor this
const MAX_STACK_SIZE: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CallStack {
    pub(crate) stack: BiDirectionalShiftRegister,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BiDirectionalShiftRegister {
    pub(crate) stack: [Address; MAX_STACK_SIZE],
    enabled: bool,
    pub(crate) stack_top: Option<Address>,
    write_buffer: Option<Address>,
}

impl Register for BiDirectionalShiftRegister {
    type WriteInformation = Address;

    fn enable(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.write_buffer = None;
        }
    }

    fn clock(&mut self) {
        let Some(address) = self.write_buffer.take() else {
            return; // No write scheduled
        };
        for i in (1..MAX_STACK_SIZE).rev() {
            self.stack[i] = self.stack[i - 1];
        }
        self.stack[0] = address;
        self.stack_top = Some(address);
    }

    fn schedule_write(&mut self, write_info: Self::WriteInformation) {
        if self.enabled {
            self.write_buffer = Some(write_info);
        }
    }
}

impl Default for CallStack {
    fn default() -> Self {
        CallStack::new()
    }
}

impl CallStack {
    pub(crate) fn new() -> Self {
        CallStack {
            stack: BiDirectionalShiftRegister {
                stack: [Address::default(); MAX_STACK_SIZE],
                enabled: true,
                stack_top: None,
                write_buffer: None,
            },
        }
    }

    pub(crate) fn push(&mut self, address: Address) {
        self.stack.schedule_write(address);
    }

    pub(crate) fn pop(&mut self) -> Option<Address> {
        if let Some(top) = self.stack.stack_top {
            for i in 0..MAX_STACK_SIZE - 1 {
                self.stack.stack[i] = self.stack.stack[i + 1];
            }
            self.stack.stack[MAX_STACK_SIZE - 1] = Address::default();
            self.stack.stack_top = if top == Address::default() {
                None
            } else {
                Some(self.stack.stack[0])
            };
            Some(top)
        } else {
            None
        }
    }
}
