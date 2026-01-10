use crate::Word;

#[derive(Clone, Copy, Default, Hash)]
#[repr(transparent)]
pub struct Limb(pub Word);
