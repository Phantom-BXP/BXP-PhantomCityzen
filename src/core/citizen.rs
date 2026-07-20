//! Citoyen numérique : générateur déterministe multi-pays.
//! Toutes les dates sont dans le passé (avant 2026).

use super::prng::SplitMix64;
use super::life_history::{Education, Employment, Housing};
use super::country::Country;

const CURRENT_YEAR: u16 = 2026; // Année de référence pour les vérifications

// ─── STRUCTURE DU CITOYEN ───

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Citizen {
    pub country: Country,
    pub first_name: [u8; 32],
    pub last_name: [u8; 32],
    pub birth_year: u16,
    pub birth_timestamp: u64,
    pub city: [u8; 32],
    pub postal_code: u32,
    pub education: Education,
    pub employment: Employment,
    pub housing: Housing,
}

impl Citizen {
    // ─── Helpers d'affichage ───

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

    pub fn education_str(&self) -> (&str, &str, u16) {
        (
            core::str::from_utf8(&self.education.degree).unwrap_or("???").trim_end_matches('\0'),
            core::str::from_utf8(&self.education.institution).unwrap_or("???").trim_end_matches('\0'),
            self.education.graduation_year,
        )
    }

    pub fn employment_str(&self) -> (&str, &str, u32, u16) {
        (
            core::str::from_utf8(&self.employment.employer).unwrap_or("???").trim_end_matches('\0'),
            core::str::from_utf8(&self.employment.position).unwrap_or("???").trim_end_matches('\0'),
            self.employment.annual_salary,
            self.employment.start_year,
        )
    }

    pub fn housing_str(&self) -> (u8, u16, u16) {
        (self.housing.housing_type, self.housing.monthly_rent, self.housing.entry_year)
    }

    // ─── GÉNÉRATEUR ───

    pub fn try_generate(seed: u64) -> Result<Self, &'static str> {
        let mut rng = SplitMix64::new(seed);
        let country = Country::from_seed(seed);
        let dict = country.dictionaries();

        // Paramètres culturels
        let grad_age = country.graduation_age();
        let (min_rent, max_rent) = country.rent_range();
        let (_min_sal, _max_sal) = country.salary_range();

        // 1. Prénom
        let idx = (rng.next_u32() as usize) % dict.first_names.len();
        let first_name = dict.first_names[idx];

        // 2. Nom
        let idx = (rng.next_u32() as usize) % dict.last_names.len();
        let last_name = dict.last_names[idx];

        // 3. Date de naissance (entre 1940 et 1998)
        let year = 1940 + (rng.next_u32() % 59); // 1940..1998
        let month = 1 + (rng.next_u32() % 12);
        let day = 1 + (rng.next_u32() % 28);
        let birth_timestamp = (year * 365 + month * 30 + day) as u64 * 86400;

        // 4. Ville
        let idx = (rng.next_u32() as usize) % dict.cities.len();
        let (city_name, postal_code) = dict.cities[idx];

        // 5. Éducation (diplôme entre 18 et 26 ans, mais avant 2026)
        let grad_age_u32 = u32::from(grad_age);
        let mut graduation_year = year + grad_age_u32 + (rng.next_u32() % 5);
        if graduation_year > (CURRENT_YEAR - 1) as u32 {
            graduation_year = (CURRENT_YEAR - 1) as u32;
        }
        let graduation_year = graduation_year as u16;

        let degree_idx = (rng.next_u32() as usize) % super::life_history::DEGREES.len();
        let university_idx = (rng.next_u32() as usize) % dict.universities.len();
        let mut degree_arr = [0u8; 64];
        let mut uni_arr = [0u8; 64];
        copy_str_to_array(&mut degree_arr, super::life_history::DEGREES[degree_idx])?;
        copy_str_to_array(&mut uni_arr, dict.universities[university_idx])?;
        let education = Education {
            degree: degree_arr,
            institution: uni_arr,
            graduation_year,
        };

        // 6. Emploi (début après diplôme, avant 2026)
        let mut start_year = graduation_year as u32 + (rng.next_u32() % 3);
        if start_year > (CURRENT_YEAR - 1) as u32 {
            start_year = (CURRENT_YEAR - 1) as u32;
        }
        let start_year = start_year as u16;

        let employer_idx = (rng.next_u32() as usize) % dict.employers.len();
        let (employer_str, (emp_min, emp_max)) = dict.employers[employer_idx];
        let salary = emp_min + (rng.next_u32() % (emp_max - emp_min));
        let mut employer_arr = [0u8; 64];
        let mut position_arr = [0u8; 64];
        copy_str_to_array(&mut employer_arr, employer_str)?;
        let positions = [
            "Développeur", "Consultant", "Ingénieur", "Analyste",
            "Chef de projet", "Architecte", "Responsable", "Directeur",
        ];
        let pos_idx = (rng.next_u32() as usize) % positions.len();
        copy_str_to_array(&mut position_arr, positions[pos_idx])?;
        let employment = Employment {
            employer: employer_arr,
            position: position_arr,
            annual_salary: salary,
            start_year,
        };

        // 7. Logement
        let housing_type = (rng.next_u32() % 4) as u8;
        let max_rent_calc = (salary as f32 * 0.33 / 12.0) as u16;
        let min_rent_eff = min_rent.min(max_rent_calc);
        let max_rent_eff = max_rent.min(max_rent_calc);
        let rent = if max_rent_eff > min_rent_eff {
            let range = (max_rent_eff - min_rent_eff) as u32;
            min_rent_eff + (rng.next_u32() % range) as u16
        } else {
            min_rent_eff
        };
        let housing = Housing {
            housing_type,
            monthly_rent: rent,
            entry_year: start_year,
        };

        // 8. Copie des chaînes
        let mut first_name_arr = [0u8; 32];
        let mut last_name_arr = [0u8; 32];
        let mut city_arr = [0u8; 32];

        copy_str_to_array(&mut first_name_arr, first_name)?;
        copy_str_to_array(&mut last_name_arr, last_name)?;
        copy_str_to_array(&mut city_arr, city_name)?;

        Ok(Self {
            country,
            first_name: first_name_arr,
            last_name: last_name_arr,
            birth_year: year as u16,
            birth_timestamp,
            city: city_arr,
            postal_code,
            education,
            employment,
            housing,
        })
    }
}

// ─── UTILITAIRE ─────────────────────────────────────────

fn copy_str_to_array<const LEN: usize>(dest: &mut [u8; LEN], src: &str) -> Result<(), &'static str> {
    let bytes = src.as_bytes();
    if bytes.len() >= LEN {
        return Err("Chaîne trop longue");
    }
    dest[..bytes.len()].copy_from_slice(bytes);
    dest[bytes.len()] = 0;
    Ok(())
}

// ─── TESTS DE COHÉRENCE ─────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graduation_after_min_age() {
        let citizen = Citizen::try_generate(0x1234).unwrap();
        let birth_year = citizen.birth_year;
        let grad_age = citizen.country.graduation_age();
        assert!(citizen.education.graduation_year >= birth_year + grad_age);
    }

    #[test]
    fn test_employment_after_graduation() {
        let citizen = Citizen::try_generate(0x5678).unwrap();
        assert!(citizen.employment.start_year >= citizen.education.graduation_year);
    }

    #[test]
    fn test_rent_affordable() {
        let citizen = Citizen::try_generate(0x9ABC).unwrap();
        let max_rent = (citizen.employment.annual_salary as f32 * 0.33 / 12.0) as u16;
        assert!(citizen.housing.monthly_rent <= max_rent);
    }

    #[test]
    fn test_all_dates_in_past() {
        let citizen = Citizen::try_generate(0xDEADBEEF).unwrap();
        assert!(citizen.birth_year < CURRENT_YEAR);
        assert!(citizen.education.graduation_year < CURRENT_YEAR);
        assert!(citizen.employment.start_year < CURRENT_YEAR);
        assert!(citizen.housing.entry_year < CURRENT_YEAR);
    }

    #[test]
    fn test_determinism() {
        let seed = 0xDEADBEEF;
        let c1 = Citizen::try_generate(seed).unwrap();
        let c2 = Citizen::try_generate(seed).unwrap();
        assert_eq!(c1.first_name, c2.first_name);
        assert_eq!(c1.last_name, c2.last_name);
        assert_eq!(c1.birth_year, c2.birth_year);
        assert_eq!(c1.city, c2.city);
        assert_eq!(c1.education.graduation_year, c2.education.graduation_year);
        assert_eq!(c1.employment.annual_salary, c2.employment.annual_salary);
        assert_eq!(c1.housing.monthly_rent, c2.housing.monthly_rent);
    }

    #[test]
    fn test_country_consistency() {
        let seed = 0xDEADBEEF;
        let c1 = Citizen::try_generate(seed).unwrap();
        let c2 = Citizen::try_generate(seed).unwrap();
        assert_eq!(c1.country, c2.country);
    }
}
