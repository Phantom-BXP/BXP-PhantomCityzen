//! Citoyen numérique : structure et générateur déterministe.
//! Version avec dictionnaires statiques intégrés (200 prénoms, 200 noms, 50 villes).

use super::prng::SplitMix64;

// ─── DICTIONNAIRES STATIQUES (intégrés au binaire) ───

/// 200 prénoms les plus courants en France (données INSEE approximatives).
const FIRST_NAMES: &[&str] = &[
    "Thomas", "Julie", "Nicolas", "Sarah", "Pierre", "Marie", "Jean", "Sophie",
    "Lucas", "Emma", "Louis", "Chloé", "Hugo", "Inès", "Maxime", "Léa",
    "Arthur", "Camille", "Jules", "Alice", "Adrien", "Mila", "Raphaël", "Zoé",
    "Clément", "Lina", "Théo", "Jeanne", "Nathan", "Élisa", "Paul", "Nina",
    "Mathéo", "Louise", "Tom", "Julia", "Noah", "Rose", "Ethan", "Anna",
    "Quentin", "Léonie", "Alexis", "Lola", "Benjamin", "Victoire", "Antoine", "Romane",
    "Baptiste", "Agathe", "Vincent", "Ava", "Gabriel", "Léna", "Alexandre", "Éva",
    "Julien", "Lou", "Romain", "Iris", "David", "Zélie", "Sacha", "Alba",
    "Florian", "Maeva", "Nolan", "Capucine", "Raphaël", "Elena", "Mathis", "Louane",
    "Arnaud", "Lucie", "Emilien", "Clara", "Cédric", "Maëlys", "Dorian", "Léane",
    "Guillaume", "Juliette", "Axel", "Salomé", "Kylian", "Élise", "François", "Agathe",
    "Liam", "Margaux", "Damien", "Alicia", "Jean-Baptiste", "Garance", "Philippe", "Esther",
    "Tristan", "Alix", "Gabin", "Manon", "Rémi", "Maëlle", "Thibault", "Suzanne",
    "Bastien", "Charline", "Grégory", "Margot", "Raphaël", "Adèle", "Alexandre", "Thaïs",
    "Marius", "Ariane", "Benoît", "Élise", "Étienne", "Eugénie", "Fabien", "Éléonore",
    "Félix", "Héloïse", "Gaspard", "Isaline", "Hubert", "Joséphine", "Ismaël", "Constance",
    "Jacques", "Léopoldine", "Jean", "Lison", "Joseph", "Mélanie", "Jules", "Marion",
    "Laurent", "Michèle", "Léandre", "Nora", "Martin", "Olympe", "Matthieu", "Prune",
    "Maxence", "Roxane", "Maurice", "Sibylle", "Noël", "Thaïs", "Olivier", "Violette",
    "Pascal", "Yvonne", "Philippe", "Elsa", "Pierre", "Agnès", "Quentin", "Bérengère",
    "Rémy", "Céleste", "Sébastien", "Diane", "Thierry", "Fleur", "Ulysse", "Gisèle",
    "Valentin", "Hortense", "Victor", "Irène", "Xavier", "Jacqueline", "Yann", "Léa",
    "Zacharie", "Madeleine", "Alain", "Marceline", "Bernard", "Odette", "Christian", "Paulette",
    "Denis", "Renée", "Éric", "Simone", "Frédéric", "Yvette", "Gérard", "Zélia",
    "Hervé", "Annette", "Jean-Claude", "Berthe", "Michel", "Clémence", "Patrick", "Cécile",
    "Roger", "Lucienne", "Jean-Michel", "Marguerite", "Serge", "Géraldine", "Daniel", "Françoise",
    "Guy", "Solange", "André", "Thérèse", "Robert", "Geneviève", "Marcel", "Yolande",
    "Jacques", "Jeannine", "Louis", "Germaine", "Jean-Pierre", "Raymonde", "Georges", "Marthe",
];

/// 200 noms de famille les plus courants en France.
const LAST_NAMES: &[&str] = &[
    "Martin", "Bernard", "Dubois", "Thomas", "Robert", "Richard", "Petit", "Durand",
    "Leroy", "Moreau", "Simon", "Laurent", "Michel", "Garcia", "David", "Bertrand",
    "Roux", "Vincent", "Fournier", "Morel", "Girard", "Andre", "Lefevre", "Mercier",
    "Lefebvre", "Dupont", "Lambert", "Bonnet", "Francois", "Martinez", "Legrand", "Garnier",
    "Faure", "Rousseau", "Renaud", "Roy", "Michaud", "Dumas", "Marie", "Fontaine",
    "Marchand", "Adam", "Mathieu", "Clement", "Colin", "Arnaud", "Boucher", "Meyer",
    "Brun", "Caron", "Morin", "Gautier", "Lopez", "Barre", "Chevalier", "Germain",
    "Perez", "Dufour", "Guillot", "Leclerc", "Benoit", "Joly", "Riviere", "Fabre",
    "Fernandez", "Bourgeois", "Coulon", "Moulin", "Pons", "Lemoine", "Delacroix", "Blanchard",
    "Lacroix", "Roussel", "Huet", "Garcia", "Deschamps", "Roland", "Perrin", "Delmas",
    "Tanguy", "Olivier", "Vidal", "Le Goff", "Petitjean", "Giraud", "Laine", "Bertin",
    "Aubry", "Carpentier", "Remy", "Legendre", "Dupuis", "Gilles", "Lucas", "Leclercq",
    "Carlier", "Hamon", "Roche", "Maillet", "Briand", "Grenier", "Tessier", "Lefranc",
    "Le Gall", "Poulain", "Dufresne", "Monnier", "Lemoine", "Guillon", "Delattre", "Legros",
    "Maillard", "Bailly", "Neveu", "Lemoine", "Delaunay", "Blin", "Parisot", "Goubert",
    "Dupont", "Leblanc", "Dufresne", "Chemin", "Barbier", "Vasseur", "Leroux", "Leduc",
    "Maurice", "Lejeune", "Masson", "Pichon", "Breton", "Boutin", "Leclerc", "Morel",
    "Chauvin", "Fontaine", "Peltier", "Guibert", "Roussel", "Bouvier", "Pereira", "Bourdon",
    "Riviere", "Berger", "Gosselin", "Rey", "Beau", "Marchal", "Guyot", "Guillou",
    "Duval", "Camus", "Moulin", "Bodin", "Beauvais", "Leroy", "Gilbert", "Lecomte",
    "Boutin", "Dupuy", "Charles", "Gautier", "Leclercq", "Gillet", "Lefevre", "Dumont",
    "Legrand", "Bouillon", "Chevalier", "Joubert", "Garnier", "Chauvet", "Benoit", "Peyron",
    "Lhermitte", "Guillou", "Benoist", "Vernier", "Riviere", "Verdier", "Lacroix", "Morvan",
    "Devaux", "Guion", "Rocher", "Pottier", "Marechal", "Morin", "Bourgeois", "Langlois",
    "Legras", "Roger", "Vallée", "Millet", "Lefevre", "Lemoine", "Dupuis", "Giraud",
];

/// 50 villes avec leur code postal (INSEE simplifié).
const CITIES: &[(&str, u32)] = &[
    ("Paris", 75001), ("Marseille", 13001), ("Lyon", 69001), ("Toulouse", 31000),
    ("Nice", 06000), ("Nantes", 44000), ("Montpellier", 34000), ("Strasbourg", 67000),
    ("Bordeaux", 33000), ("Lille", 59000), ("Rennes", 35000), ("Reims", 51100),
    ("Le Havre", 76600), ("Saint-Étienne", 42000), ("Toulon", 83000), ("Grenoble", 38000),
    ("Dijon", 21000), ("Angers", 49000), ("Nîmes", 30000), ("Villeurbanne", 69100),
    ("Saint-Denis", 93200), ("Le Mans", 72000), ("Aix-en-Provence", 13100), ("Clermont-Ferrand", 63000),
    ("Brest", 29200), ("Limoges", 87000), ("Tours", 37000), ("Amiens", 80000),
    ("Perpignan", 66000), ("Metz", 57000), ("Besançon", 25000), ("Boulogne-Billancourt", 92100),
    ("Orléans", 45000), ("Nancy", 54000), ("Roubaix", 59100), ("Argenteuil", 95100),
    ("Tourcoing", 59200), ("Montreuil", 93100), ("Caen", 14000), ("Poitiers", 86000),
    ("Nanterre", 92000), ("Versailles", 78000), ("Courbevoie", 92400), ("Vitry-sur-Seine", 94400),
    ("Créteil", 94000), ("Colombes", 92700), ("Pau", 64000), ("Rueil-Malmaison", 92500),
    ("Aubervilliers", 93300), ("Drancy", 93700),
];

// ─── STRUCTURE DU CITOYEN ───

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Citizen {
    pub first_name: [u8; 32],
    pub last_name: [u8; 32],
    pub birth_timestamp: u64,
    pub city: [u8; 32],
    pub postal_code: u32,
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

    // ─── GÉNÉRATEUR ───

    pub fn try_generate(seed: u64) -> Result<Self, &'static str> {
        let mut rng = SplitMix64::new(seed);

        // 1. Prénom
        let idx = (rng.next_u32() as usize) % FIRST_NAMES.len();
        let first_name = FIRST_NAMES[idx];

        // 2. Nom
        let idx = (rng.next_u32() as usize) % LAST_NAMES.len();
        let last_name = LAST_NAMES[idx];

        // 3. Date de naissance (entre 1940 et 2010)
        let year = 1940 + (rng.next_u32() % 71);
        let month = 1 + (rng.next_u32() % 12);
        let day = 1 + (rng.next_u32() % 28); // On évite les erreurs de jours
        let birth_timestamp = (year * 365 + month * 30 + day) as u64 * 86400;

        // 4. Ville
        let idx = (rng.next_u32() as usize) % CITIES.len();
        let (city_name, postal_code) = CITIES[idx];

        // Copie dans les tableaux fixes
        let mut first_name_arr = [0u8; 32];
        let mut last_name_arr = [0u8; 32];
        let mut city_arr = [0u8; 32];

        copy_str_to_array(&mut first_name_arr, first_name)?;
        copy_str_to_array(&mut last_name_arr, last_name)?;
        copy_str_to_array(&mut city_arr, city_name)?;

        Ok(Self {
            first_name: first_name_arr,
            last_name: last_name_arr,
            birth_timestamp,
            city: city_arr,
            postal_code: *postal_code,
        })
    }
}

// ─── UTILITAIRE ───

/// Copie une chaîne dans un tableau de 32 octets (null-terminé).
fn copy_str_to_array(dest: &mut [u8; 32], src: &str) -> Result<(), &'static str> {
    let bytes = src.as_bytes();
    if bytes.len() >= 32 {
        return Err("Chaîne trop longue");
    }
    dest[..bytes.len()].copy_from_slice(bytes);
    dest[bytes.len()] = 0; // null-terminated
    Ok(())
}
