use bxp_phantom_citizen::core::citizen::Citizen;

fn main() {
    let seed: u64 = 0x123456789ABCDEF;
    match Citizen::try_generate(seed) {
        Ok(citizen) => {
            println!("=== BXP-PhantomCitizen ===");
            println!("Pays : {:?}", citizen.country);
            println!("Nom : {} {}", citizen.first_name_str(), citizen.last_name_str());
            println!("Année de naissance : {}", citizen.birth_year);
            println!("Ville : {}", citizen.city_str());
            let (degree, uni, grad_year) = citizen.education_str();
            println!("Diplôme : {} de {} ({})", degree, uni, grad_year);
            let (employer, position, salary, start) = citizen.employment_str();
            println!("Emploi : {} en tant que {} ({}€/an) depuis {}", employer, position, salary, start);
            let (h_type, rent, entry) = citizen.housing_str();
            let h_type_str = match h_type { 0 => "Studio", 1 => "T2", 2 => "T3", _ => "Maison" };
            println!("Logement : {} ({}€/mois) depuis {}", h_type_str, rent, entry);
        }
        Err(e) => eprintln!("Erreur : {}", e),
    }
}
