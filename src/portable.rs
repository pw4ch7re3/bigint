use crate::{
    Condition, ConditionalMove, ConditionalMoveEq,
    is_nonzero,
    impl_conditional_move, impl_conditional_move_eq, impl_conditional_move_traits,
};
use core::{hint::black_box, u64};

impl ConditionalMove for u64 {
    #[inline]
    fn conditional_move_zero(&mut self, value: &Self, condition: Condition) {
        let mask = black_box((1 ^ is_nonzero!(condition, u8::BITS) as u64).wrapping_sub(1));
        *self = (*self & mask) | (*value & !mask);
    }
    
    #[inline]
    fn conditional_move_nonzero(&mut self, value: &Self, condition: Condition) {
        let mask = black_box((is_nonzero!(condition, u8::BITS) as u64).wrapping_sub(1));
        *self = (*self & mask) | (*value & !mask);
    }
}

impl_conditional_move!(u16, u32);

impl ConditionalMove for u8 {
    #[inline]
    fn conditional_move_zero(&mut self, value: &Self, condition: Condition) {
        let mut flag = u16::from(*self);
        flag.conditional_move_zero(&u16::from(*value), condition);
        *self = (flag & 0xff) as u8;
    }

    #[inline]
    fn conditional_move_nonzero(&mut self, value: &Self, condition: Condition) {
        let mut flag = u16::from(*self);
        flag.conditional_move_nonzero(&u16::from(*value), condition);
        *self = (flag & 0xff) as u8;
    }
}

impl ConditionalMove for u128 {
    #[inline]
    fn conditional_move_zero(&mut self, value: &Self, condition: Condition) {
        let mut hi  = (*self >> 64) as u64;
        let mut lo = (*self & u128::from(u64::MAX)) as u64;
        
        hi.conditional_move_zero(&((*value >> 64) as u64), condition);
        lo.conditional_move_zero(&((*value & u128::from(u64::MAX)) as u64), condition);

        *self = (u128::from(hi) << 64) | u128::from(lo);
    }

    #[inline]
    fn conditional_move_nonzero(&mut self, value: &Self, condition: Condition) {
        let mut hi  = (*self >> 64) as u64;
        let mut lo = (*self & u128::from(u64::MAX)) as u64;
        
        hi.conditional_move_nonzero(&((*value >> 64) as u64), condition);
        lo.conditional_move_nonzero(&((*value & u128::from(u64::MAX)) as u64), condition);

        *self = (u128::from(hi) << 64) | u128::from(lo);
    }
}

impl ConditionalMoveEq for u64 {
    #[inline]
    fn conditional_move_eq(&self, rhs: &Self, input: Condition, output: &mut Condition) {
        let flag = is_nonzero!(self ^ rhs, u64::BITS) as u8;
        output.conditional_move_nonzero(&input, black_box(flag ^ 1));
    }

    #[inline]
    fn conditional_move_ne(&self, rhs: &Self, input: Condition, output: &mut Condition) {
        let flag = is_nonzero!(self ^ rhs, u64::BITS) as u8;
        output.conditional_move_nonzero(&input, black_box(flag));    
    }
}

impl_conditional_move_eq!(u16, u32);

impl ConditionalMoveEq for u8 {
    #[inline]
    fn conditional_move_eq(&self, rhs: &Self, input: Condition, output: &mut Condition) {
        u16::from(*self).conditional_move_eq(&u16::from(*rhs), input, output);
    }

    #[inline]
    fn conditional_move_ne(&self, rhs: &Self, input: Condition, output: &mut Condition) {
        u16::from(*self).conditional_move_ne(&u16::from(*rhs), input, output);
    }
}

impl ConditionalMoveEq for u128 {
    #[inline]
    fn conditional_move_eq(&self, rhs: &Self, input: Condition, output: &mut Condition) {
        let hi = (*self >> 64) as u64;
        let lo = (*self & u128::from(u64::MAX)) as u64;
        let mut flag = 1;

        hi.conditional_move_eq(&((*rhs >> 64) as u64), 0, &mut flag);
        lo.conditional_move_eq(&((*rhs & u128::from(u64::MAX)) as u64), 0, &mut flag);
        flag.conditional_move_eq(&1, input, output);
    }

    #[inline]
    fn conditional_move_ne(&self, rhs: &Self, input: Condition, output: &mut Condition) {
        let hi = (*self >> 64) as u64;
        let lo = (*self & u128::from(u64::MAX)) as u64;
        let mut flag = 1;

        hi.conditional_move_ne(&((*rhs >> 64) as u64), 0, &mut flag);
        lo.conditional_move_ne(&((*rhs & u128::from(u64::MAX)) as u64), 0, &mut flag);
        flag.conditional_move_ne(&0, input, output);
    }
}

impl_conditional_move_traits!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, i128 => u128);

impl<T: ConditionalMoveEq> ConditionalMoveEq for [T] {
    fn conditional_move_eq(&self, rhs: &Self, input: Condition, output: &mut Condition) {
        let mut flag = 1;
        self.conditional_move_ne(rhs, 0, &mut flag);
        flag.conditional_move_eq(&1, input, output);
    }

    fn conditional_move_ne(&self, rhs: &Self, input: Condition, output: &mut Condition) {
        if self.len() != rhs.len() {
            *output = input;
        } else {
            for (a, b) in self.iter().zip(rhs.iter()) {
                a.conditional_move_ne(b, input, output);
            }
        }
    }
}
