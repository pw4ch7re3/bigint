#[macro_export]
macro_rules! is_nonzero {
    ( $value:expr, $bits:expr ) => {
        ($value | $value.wrapping_neg()) >> ($bits - 1)
    };
}

#[macro_export]
macro_rules! impl_conditional_move {
    ( $($uint:ty),+ ) => {
        $(
            impl ConditionalMove for $uint {
                #[inline]
                fn conditional_move_zero(&mut self, value: &Self, condition: Condition) {
                    let mut flag = *self as u64;
                    flag.conditional_move_zero(&(*value as u64), condition);
                    *self = flag as $uint;
                }
    
                #[inline]
                fn conditional_move_nonzero(&mut self, value: &Self, condition: Condition) {
                    let mut flag = *self as u64;
                    flag.conditional_move_nonzero(&(*value as u64), condition);
                    *self = flag as $uint;
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_conditional_move_eq {
    ( $($uint:ty),+ ) => {
        $(
            impl ConditionalMoveEq for $uint {
                #[inline]
                fn conditional_move_eq(&self, rhs: &Self, input: Condition, output: &mut Condition) {
                    (*self as u64).conditional_move_eq(&(*rhs as u64), input, output);
                }

                #[inline]
                fn conditional_move_ne(&self, rhs: &Self, input: Condition, output: &mut Condition) {
                    (*self as u64).conditional_move_ne(&(*rhs as u64), input, output);
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_conditional_move_traits {
    ( $($int:ty => $uint:ty),+ ) => {
        $(
            impl ConditionalMove for $int {
                #[inline]
                fn conditional_move_zero(&mut self, value: &Self, condition: Condition) {
                    let mut flag = *self as $uint;
                    flag.conditional_move_zero(&(*value as $uint), condition);
                    *self = flag as $int;
                }
    
                #[inline]
                fn conditional_move_nonzero(&mut self, value: &Self, condition: Condition) {
                    let mut flag = *self as $uint;
                    flag.conditional_move_nonzero(&(*value as $uint), condition);
                    *self = flag as $int;
                }
            }

            impl ConditionalMoveEq for $int {
                #[inline]
                fn conditional_move_eq(&self, rhs: &Self, input: Condition, output: &mut Condition) {
                    (*self as $uint).conditional_move_eq(&(*rhs as $uint), input, output);
                }

                #[inline]
                fn conditional_move_ne(&self, rhs: &Self, input: Condition, output: &mut Condition) {
                    (*self as $uint).conditional_move_ne(&(*rhs as $uint), input, output);
                }
            }
        )+
    };
}
