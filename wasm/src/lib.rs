use emulator::{
    cpu::Cpu,
    memory::{memory_hexdump, Memory},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu<Memory>,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(Memory::default()),
        }
    }

    pub fn load(&mut self, data: Vec<u8>) {
        self.cpu.load(&data);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn execute(&mut self) {
        self.cpu.execute();
    }

    pub fn memory_hexdump(&self, start: u16, end: u16) -> String {
        memory_hexdump(&self.cpu.memory, start, end)
    }

    pub fn cpu_status(&self) -> String {
        format!("{}", self.cpu)
    }
}
