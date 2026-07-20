//! Générateur déterministe SplitMix64.
//! Utilisé pour transformer une graine en une séquence de nombres
//! qui alimenteront tous les choix du générateur d'identité.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
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

impl FnMut<()> for SplitMix64 {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> u64 {
        self.next_u64()
    }
}

Version stable sans FnMut (recommandée) :

```rust
impl SplitMix64 {
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

// Note : L'implémentation de FnMut est optionnelle mais pratique
// pour l'API `sample(&mut self, rng: &mut impl FnMut() -> u64)`.
// Cependant, l'implémentation de FnMut pour une structure est instable
// sur stable Rust. On utilisera plutôt un wrapper ou une fonction séparée.
// Je propose donc de ne pas implémenter FnMut, mais de fournir un adaptateur.
