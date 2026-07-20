//! Générateur déterministe SplitMix64.
//! Implémentation de référence, sans dépendance.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    #[inline(always)]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    #[inline(always)]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    #[inline(always)]
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
}
