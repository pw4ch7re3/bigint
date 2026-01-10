use core::ops::{
    Not,
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
};

#[derive(Clone, Copy, Debug)]
pub struct Choice(pub(crate) u8);

impl Choice {
    /// true
    pub const TRUE: Self = Self(1);
    /// false
    pub const FALSE: Self = Self(0);

    #[inline]
    pub const fn and(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    #[inline]
    pub const fn or(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    #[inline]
    pub const fn xor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    #[inline]
    pub const fn not(self) -> Self {
        self.xor(Self(1))
    }

    #[inline]
    pub const fn ne(self, rhs: Self) -> Self {
        self.xor(rhs)
    }

    #[inline]
    pub const fn eq(self, rhs: Self) -> Self {
        self.ne(rhs).not()
    }

    #[inline]
    pub const fn from_u8(value: u8) -> Self {
        Self(value & 0x01)
    }

    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        Self((value & 0x01) as u8)
    }

    #[inline]
    pub const fn from_u32(value: u32) -> Self {
        Self((value & 0x01) as u8)
    }

    #[inline]
    pub const fn from_u64(value: u64) -> Self {
        Self((value & 0x01) as u8)
    }

    #[inline]
    pub const fn from_u128(value: u128) -> Self {
        Self((value & 0x01) as u8)
    }

    pub const fn to_u8(self) -> u8 {
        core::hint::black_box(self.0)
    }

    pub const fn to_bool(self) -> bool {
        self.to_u8() != 0
    }

    pub const fn vt_to_u8(self) -> u8 {
        self.0
    }

    pub const fn vt_to_bool(self) -> bool {
        self.0 != 0
    }
}

impl Not for Choice {
    type Output = Self;

    #[inline]
    fn not(self) -> <Self as Not>::Output {
        self.not()
    }
}

impl BitAnd for Choice {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> <Self as BitAnd>::Output {
        self.and(rhs)
    }
}

impl BitAndAssign for Choice {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for Choice {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
        self.or(rhs)
    }
}

impl BitOrAssign for Choice {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for Choice {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> <Self as BitXor>::Output {
        self.xor(rhs)
    }
}

impl BitXorAssign for Choice {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl From<Choice> for u8 {
    fn from(value: Choice) -> Self {
        value.to_u8()
    }
}

impl From<Choice> for bool {
    fn from(value: Choice) -> Self {
        value.to_bool()
    }
}

#[cfg(feature = "subtle")]
impl From<subtle::Choice> for Choice {
    #[inline]
    fn from(value: subtle::Choice) -> Self {
        Self(value.unwrap_u8())
    }
}

#[cfg(feature = "subtle")]
impl From<Choice> for subtle::Choice {
    #[inline]
    fn from(value: Choice) -> Self {
        Self::from(value.0)
    }
}
