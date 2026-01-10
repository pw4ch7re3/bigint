use super::Limb;
use core::ops::{ BitOr, BitOrAssign, };

impl Limb {
    #[inline(always)]
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOr for Limb {
    type Output = Self;

    fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
        self.bitor(rhs)
    }
}

impl BitOrAssign for Limb {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<&Limb> for Limb {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.0 |= rhs.0;
    }
}
