#![no_std]
#![deny(unsafe_code)]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

mod portable;
mod choice;
mod ct_option;
mod traits;

pub use choice::Choice;
pub use ct_option::CtOption;

#[macro_use]
mod macros;

pub type Condition = u8;

pub trait ConditionalMove {
    fn conditional_move_zero(&mut self, value: &Self, condition: Condition);
    fn conditional_move_nonzero(&mut self, value: &Self, condition: Condition) {
        self.conditional_move_zero(value, !condition);
    }
}

pub trait ConditionalMoveEq {
    fn conditional_move_eq(&self, rhs: &Self, input: Condition, output: &mut Condition);
    fn conditional_move_ne(&self, rhs: &Self, input: Condition, output: &mut Condition) {
        let mut flag = 1;
        self.conditional_move_eq(rhs, 0, &mut flag);
        flag.conditional_move_eq(&1, input, output);
    }
}
