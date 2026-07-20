//! Binaire de test pour BXP-PhantomCitizen.
//! Nécessite l'activation de `std` pour l'affichage.

use bxp_phantom_citizen::core::citizen::Citizen;

fn main() {
    let seed: u64 = 0x123456789ABCDEF;
    match Citizen::try_generate(seed) {
        Ok(citizen) => {
            println!("=== BXP-PhantomCitizen ===");
            println!("Prénom  : {}", citizen.first_name_str());
            println!("Nom      : {}", citizen.last_name_str());
            println!("Ville    : {} ({:05})", citizen.city_str(), citizen.postal_code);
            println!("Timestamp: {}", citizen.birth_timestamp);
        }
        Err(e) => eprintln!("Erreur: {}", e),
    }
}
