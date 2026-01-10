use crate::Limb;

#[derive(Clone, Copy, Hash)]
pub struct Uint<const LIMBS: usize> {
    pub(crate) limbs: [Limb; LIMBS],
}
