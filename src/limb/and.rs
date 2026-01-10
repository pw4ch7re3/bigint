use super::Limb;
use core::ops::{ BitAnd, BitAndAssign, };

impl Limb {
    #[inline(always)]
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitAnd for Limb {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> <Self as BitAnd>::Output {
        self.bitand(rhs)
    }
}

impl BitAndAssign for Limb {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<&Limb> for Limb {
    fn bitand_assign(&mut self, rhs: &Self) {
        self.0 &= rhs.0;
    }
}
