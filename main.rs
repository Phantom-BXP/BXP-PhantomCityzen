// src/main.rs
#![no_std]
#![no_main]

use bxp_phantom_citizen::core::citizen::Citizen;

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let seed = 0x123456789ABCDEF;
    match Citizen::try_generate(seed) {
        Ok(citizen) => {
            // Ici tu peux afficher, mais en no_std c'est compliqué.
            // Pour l'instant, on ne fait rien, on sait que ça compile.
            0
        }
        Err(_) => 1,
    }
}
