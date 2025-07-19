pub(crate) mod call_stack;
pub(crate) mod data_memory;
pub mod register_file;

pub(crate) trait Register {
    type WriteInformation;
    fn enable(&mut self, enabled: bool);
    fn clock(&mut self);
    fn schedule_write(&mut self, write_info: Self::WriteInformation);
}

pub use register_file::RegisterFile;

#[cfg(test)]
mod tests;
