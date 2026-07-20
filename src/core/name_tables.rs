//! Alias Table de Walker pour tirages pondérés O(1).
//! Tables pré-calculées en `const` pour zéro coût runtime.
//! Les probabilités sont stockées sur `u32` (échelle 0..=u32::MAX)
//! pour éviter les arrondis flottants et rester `no_std` friendly.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AliasTable<const N: usize> {
    prob: [u32; N],
    alias: [u16; N],
}

impl<const N: usize> AliasTable<N> {
    pub const fn from_arrays(prob: [u32; N], alias: [u16; N]) -> Self {
        Self { prob, alias }
    }

    #[inline(always)]
    pub fn sample(&self, rng: &mut impl FnMut() -> u64) -> usize {
        let i = ((rng() >> 32) as usize) % N;
        let threshold = (rng() >> 32) as u32;
        if threshold < self.prob[i] {
            i
        } else {
            self.alias[i] as usize
        }
    }
}
