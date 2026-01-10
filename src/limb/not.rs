use super::Limb;
use core::ops::Not;

impl Limb {
    #[inline(always)]
    pub const fn not(self) -> Self {
        Self(!self.0)
    }
}

impl Not for Limb {
    type Output = Self;
    
    fn not(self) -> <Self as Not>::Output {
        self.not()
    }
}
