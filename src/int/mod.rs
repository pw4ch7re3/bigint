use crate::Uint;

#[derive(Clone, Copy, Hash)]
#[repr(transparent)]
pub struct Int<const LIMBS: usize>(Uint<LIMBS>);
