use crate::{ Choice, ConditionalMoveEq, impl_ct_eq, };
use core::cmp;

#[cfg(feature = "subtle")]
use crate::CtOption;

pub trait CtEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn ct_eq(&self, rhs: &Rhs) -> Choice;
    fn ct_ne(&self, rhs: &Rhs) -> Choice {
        !self.ct_eq(rhs)
    }
}

impl_ct_eq!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl CtEq for usize {
    #[cfg(target_pointer_width = "32")]
    fn ct_eq(&self, rhs: &Self) -> Choice {
        (*self as u32).ct_eq(&(*rhs as u32))
    }

    #[cfg(target_pointer_width = "64")]
    fn ct_eq(&self, rhs: &Self) -> Choice {
        (*self as u64).ct_eq(&(*rhs as u64))
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl CtEq for isize {
    #[cfg(target_pointer_width = "32")]
    fn ct_eq(&self, rhs: &Self) -> Choice {
        (*self as i32).ct_eq(&(*rhs as i32))
    }

    #[cfg(target_pointer_width = "64")]
    fn ct_eq(&self, rhs: &Self) -> Choice {
        (*self as i64).ct_eq(&(*rhs as i64))
    }
}

impl CtEq for cmp::Ordering {
    #[inline]
    fn ct_eq(&self, rhs: &Self) -> Choice {
        (*self as i8).ct_eq(&(*rhs as i8))
    }
}

impl<T: CtEq> CtEq for [T] {
    #[inline]
    fn ct_eq(&self, rhs: &[T]) -> Choice {
        let mut flag = self.len().ct_eq(&rhs.len());
        for (a, b) in self.iter().zip(rhs.iter()) {
            flag &= a.ct_eq(b);
        }

        flag
    }
}

impl<T: CtEq, const N: usize> CtEq for [T; N] {
    #[inline]
    fn ct_eq(&self, rhs: &[T; N]) -> Choice {
        self.as_slice().ct_eq(rhs.as_slice())
    }
}

#[cfg(feature = "subtle")]
impl CtEq for subtle::Choice {
    #[inline]
    fn ct_eq(&self, rhs: &Self) -> Choice {
        self.unwrap_u8().ct_eq(&rhs.unwrap_u8())
    }
}

#[cfg(feature = "subtle")]
impl<T> CtEq for subtle::CtOption<T>
where
    T: CtEq + Default + subtle::ConditionallySelectable,
{
    #[inline]
    fn ct_eq(&self, rhs: &Self) -> Choice {
        CtOption::from(*self).ct_eq(&CtOption::from(*rhs))
    }
}
