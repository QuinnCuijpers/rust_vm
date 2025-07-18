use crate::Address;

const MAX_STACK_SIZE: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CallStack {
    stack: Vec<Address>,
}

impl Default for CallStack {
    fn default() -> Self {
        CallStack::new()
    }
}

impl CallStack {
    pub(crate) fn new() -> Self {
        CallStack {
            stack: Vec::with_capacity(MAX_STACK_SIZE),
        }
    }

    pub(crate) fn push(&mut self, address: Address) {
        self.stack.push(address);
    }

    pub(crate) fn pop(&mut self) -> Option<Address> {
        self.stack.pop()
    }
}
