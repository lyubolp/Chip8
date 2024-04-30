use crate::traits::{IRegister, IStack}
pub mod instructions {
    pub trait Instruction {
        pub fn execute(&self, registers: dyn IRegister, stack: dyn IStack);
    }
}