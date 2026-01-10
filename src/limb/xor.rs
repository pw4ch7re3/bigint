use super::Limb;
use core::ops::{ BitXor, BitXorAssign, };

impl Limb {
    #[inline(always)]
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXor for Limb {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> <Self as BitXor>::Output {
        self.bitxor(rhs)
    }
}

impl BitXorAssign for Limb {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<&Limb> for Limb {
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.0 ^= rhs.0;
    }
}
