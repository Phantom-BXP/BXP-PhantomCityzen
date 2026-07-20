//! Citoyen numérique : structure et générateur déterministe.

use super::prng::SplitMix64;
use super::alias_table::AliasTable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Citizen {
    pub first_name: [u8; 32],
    pub last_name: [u8; 32],
    pub birth_timestamp: u64,
    pub city: [u8; 32],
    pub postal_code: u32,
}

impl Citizen {
    pub fn first_name_str(&self) -> &str {
        core::str::from_utf8(&self.first_name)
            .unwrap_or("???")
            .trim_end_matches('\0')
    }

    pub fn last_name_str(&self) -> &str {
        core::str::from_utf8(&self.last_name)
            .unwrap_or("???")
            .trim_end_matches('\0')
    }

    pub fn city_str(&self) -> &str {
        core::str::from_utf8(&self.city)
            .unwrap_or("???")
            .trim_end_matches('\0')
    }

    pub fn try_generate(_seed: u64) -> Result<Self, &'static str> {
        // STUB : sera remplacé par la génération réelle quand les tables seront prêtes
        Ok(Self {
            first_name: [0u8; 32],
            last_name: [0u8; 32],
            birth_timestamp: 0,
            city: [0u8; 32],
            postal_code: 0,
        })
    }
}
