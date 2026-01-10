use crate::Choice;
use core::ops::{ Deref, DerefMut, };

#[derive(Clone, Copy, Debug)]
pub struct CtOption<T> {
    value: T,
    is_some: Choice,
}

impl<T> CtOption<T> {
    #[inline]
    pub const fn new(value: T, is_some: Choice) -> Self {
        Self { value, is_some }
    }

    #[inline]
    pub const fn some(value: T) -> Self {
        Self::new(value, Choice::TRUE)
    }

    pub fn none() -> Self
    where
        T: Default,
    {
        Self::new(Default::default(), Choice::FALSE)
    }

    #[inline]
    pub const fn to_value(self) -> T
    where
        T: Copy,
    {
        self.value
    }

    #[inline]
    pub const fn to_ref(&self) -> &T {
        &self.value
    }

    #[inline]
    pub fn into_option(self) -> Option<T> {
        if self.is_some.to_bool() {
            Some(self.value)
        } else {
            None
        }
    }

    #[inline]
    pub const fn into_option_copied(self) -> Option<T>
    where
        T: Copy,
    {
        if self.is_some.vt_to_bool() {
            Some(self.value)
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub const fn is_some(&self) -> Choice {
        self.is_some
    }

    #[must_use]
    #[inline]
    pub const fn is_none(&self) -> Choice {
        self.is_some.not()
    }

    #[inline]
    pub fn and<U>(self, mut opt_b: CtOption<U>) -> CtOption<U> {
        opt_b.is_some &= self.is_some;

        opt_b
    }

    #[inline]
    pub fn and_then<U, F>(self, f: F) -> CtOption<U>
    where
        F: FnOnce(T) -> CtOption<U>,
    {
        let mut opt = f(self.value);
        opt.is_some &= self.is_some;

        opt
    }

    #[inline]
    pub fn filter<P>(mut self, predicate: P) -> Self
    where
        P: FnOnce(&T) -> Choice,
    {
        self.is_some &= predicate(&self.value);

        self
    }

    #[inline]
    pub const fn filter_by(mut self, is_some: Choice) -> Self {
        self.is_some = self.is_some.and(is_some);

        self
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> CtOption<U>
    where
        F: FnOnce(T) -> U,
    {
        CtOption::new(f(self.value), self.is_some)
    }

    #[inline]
    pub fn zip<U>(self, rhs: CtOption<U>) -> CtOption<(T, U)> {
        CtOption { value: (self.value, rhs.value), is_some: (self.is_some & rhs.is_some), }
    }

    #[inline]
    pub fn zip_with<U, F, R>(self, rhs: CtOption<U>, f: F) -> CtOption<R>
    where
        F: FnOnce(T, U) -> R,
    {
        self.zip(rhs).map(|(a, b)| f(a, b))
    }

    #[track_caller]
    #[inline]
    pub fn expect(self, msg: &str) -> T {
        assert!(self.is_some().to_bool(), "{}", msg);

        self.value
    }

    #[track_caller]
    #[inline]
    pub const fn expect_ref(&self, msg: &str) -> &T {
        assert!(self.is_some.vt_to_bool(), "{}", msg);

        self.to_ref()
    }

    #[track_caller]
    #[inline]
    pub const fn expect_copied(self, msg: &str) -> T
    where
        T: Copy,
    {
        *self.expect_ref(msg)
    }

    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        self.into_option().ok_or(err)
    }

    #[inline]
    pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        self.ok_or(err())
    }

    #[inline]
    pub const fn as_ref(&self) -> CtOption<&T> {
        CtOption { value: &self.value, is_some: self.is_some, }
    }

    #[inline]
    pub const fn as_mut(&mut self) -> CtOption<&mut T> {
        CtOption { value: &mut self.value, is_some: self.is_some, }
    }

    #[inline]
    pub fn as_deref(&self) -> CtOption<&T::Target>
    where
        T: Deref,
    {
        self.as_ref().map(Deref::deref)
    }

    #[inline]
    pub fn as_deref_mut(&mut self) -> CtOption<&mut T::Target>
    where
        T: DerefMut,
    {
        self.as_mut().map(DerefMut::deref_mut)
    }
}

impl<T: Default> Default for CtOption<T> {
    fn default() -> Self {
        Self::none()
    }
}

impl<T> From<CtOption<T>> for Option<T> {
    fn from(value: CtOption<T>) -> Self {
        value.into_option()
    }
}

#[cfg(feature = "subtle")]
impl<T> From<CtOption<T>> for subtle::CtOption<T> {
    #[inline]
    fn from(value: CtOption<T>) -> subtle::CtOption<T> {
        subtle::CtOption::new(value.value, value.is_some.into())
    }
}
