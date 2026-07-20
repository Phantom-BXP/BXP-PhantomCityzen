//! Définition des pays supportés et de leurs dictionnaires culturels.

/// Pays supportés par le générateur.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Country {
    France,
    Germany,
    Spain,
    Italy,
    Uk,
    Mexico,
    Brazil,
    Argentina,
    Colombia,
}

impl Country {
    /// Détermine le pays à partir d'une graine (distribution uniforme).
    pub fn from_seed(seed: u64) -> Self {
        match seed % 9 {
            0 => Country::France,
            1 => Country::Germany,
            2 => Country::Spain,
            3 => Country::Italy,
            4 => Country::Uk,
            5 => Country::Mexico,
            6 => Country::Brazil,
            7 => Country::Argentina,
            _ => Country::Colombia,
        }
    }

    /// Âge moyen de fin d'études (pour la cohérence).
    pub fn graduation_age(&self) -> u16 {
        match self {
            Country::France => 22,
            Country::Germany => 24,
            Country::Spain => 23,
            Country::Italy => 24,
            Country::Uk => 21,
            Country::Mexico => 22,
            Country::Brazil => 23,
            Country::Argentina => 22,
            Country::Colombia => 23,
        }
    }

    /// Fourchette de loyer médian (min, max) en euros (converti en devise locale approximative).
    pub fn rent_range(&self) -> (u16, u16) {
        match self {
            Country::France => (400, 1200),
            Country::Germany => (450, 1400),
            Country::Spain => (350, 1000),
            Country::Italy => (380, 1100),
            Country::Uk => (500, 1500),
            // Amérique Latine (en euros équivalents, car les salaires sont en euros dans notre modèle)
            Country::Mexico => (200, 600),
            Country::Brazil => (150, 500),
            Country::Argentina => (180, 550),
            Country::Colombia => (150, 450),
        }
    }

    /// Fourchette de salaire annuel (min, max) en euros (pour harmonisation).
    pub fn salary_range(&self) -> (u32, u32) {
        match self {
            Country::France => (25000, 80000),
            Country::Germany => (28000, 90000),
            Country::Spain => (22000, 70000),
            Country::Italy => (24000, 75000),
            Country::Uk => (26000, 85000),
            Country::Mexico => (12000, 40000),
            Country::Brazil => (10000, 35000),
            Country::Argentina => (11000, 38000),
            Country::Colombia => (9000, 32000),
        }
    }

    /// Retourne les dictionnaires spécifiques au pays.
    pub fn dictionaries(&self) -> CountryDictionaries {
        match self {
            Country::France => france_dictionaries(),
            Country::Germany => germany_dictionaries(),
            Country::Spain => spain_dictionaries(),
            Country::Italy => italy_dictionaries(),
            Country::Uk => uk_dictionaries(),
            Country::Mexico => mexico_dictionaries(),
            Country::Brazil => brazil_dictionaries(),
            Country::Argentina => argentina_dictionaries(),
            Country::Colombia => colombia_dictionaries(),
        }
    }
}

/// Structure regroupant tous les dictionnaires d'un pays.
pub struct CountryDictionaries {
    pub first_names: &'static [&'static str],
    pub last_names: &'static [&'static str],
    pub cities: &'static [(&'static str, u32)],
    pub universities: &'static [&'static str],
    pub employers: &'static [(&'static str, (u32, u32))],
}

// ─── FRANCE (déjà existant, on garde) ──────────────────

fn france_dictionaries() -> CountryDictionaries {
    CountryDictionaries {
        first_names: &[
            "Thomas", "Julie", "Nicolas", "Sarah", "Pierre", "Marie", "Jean", "Sophie",
            "Lucas", "Emma", "Louis", "Chloé", "Hugo", "Inès", "Maxime", "Léa",
        ],
        last_names: &[
            "Martin", "Bernard", "Dubois", "Thomas", "Robert", "Richard", "Petit", "Durand",
            "Leroy", "Moreau", "Simon", "Laurent", "Michel", "Garcia", "David", "Bertrand",
        ],
        cities: &[
            ("Paris", 75001), ("Marseille", 13001), ("Lyon", 69001), ("Toulouse", 31000),
            ("Nice", 06000), ("Nantes", 44000), ("Montpellier", 34000), ("Strasbourg", 67000),
        ],
        universities: &[
            "Sorbonne Université", "Université Paris-Saclay", "Université de Lyon",
            "Université de Toulouse", "Université de Montpellier", "Université de Bordeaux",
        ],
        employers: &[
            ("Sopra Steria", (35000, 55000)), ("Capgemini", (38000, 58000)),
            ("Accenture", (40000, 65000)), ("Atos", (36000, 56000)),
            ("IBM", (42000, 72000)), ("Microsoft", (45000, 80000)),
        ],
    }
}

// ─── ALLEMAGNE ──────────────────────────────────────────

fn germany_dictionaries() -> CountryDictionaries {
    CountryDictionaries {
        first_names: &[
            "Hans", "Klaus", "Friedrich", "Anna", "Greta", "Ludwig", "Heinrich", "Marta",
            "Franz", "Gerda", "Otto", "Ilse", "Karl", "Ursula", "Ernst", "Hildegard",
        ],
        last_names: &[
            "Müller", "Schmidt", "Schneider", "Fischer", "Weber", "Meyer", "Wagner", "Becker",
            "Schulz", "Hoffmann", "Schäfer", "Koch", "Bauer", "Richter", "Klein", "Wolf",
        ],
        cities: &[
            ("Berlin", 10115), ("München", 80331), ("Hamburg", 20095), ("Köln", 50667),
            ("Frankfurt", 60311), ("Stuttgart", 70173), ("Düsseldorf", 40210), ("Dortmund", 44135),
        ],
        universities: &[
            "LMU Munich", "Technical University of Munich", "Heidelberg University",
            "University of Berlin", "RWTH Aachen",
        ],
        employers: &[
            ("Siemens", (40000, 70000)), ("Bosch", (38000, 65000)),
            ("Volkswagen", (42000, 75000)), ("BMW", (43000, 76000)),
            ("SAP", (45000, 80000)),
        ],
    }
}

// ─── ESPAGNE ────────────────────────────────────────────

fn spain_dictionaries() -> CountryDictionaries {
    CountryDictionaries {
        first_names: &[
            "Antonio", "Manuel", "José", "Francisco", "David", "Juan", "Javier", "Carlos",
            "María", "Carmen", "Ana", "Isabel", "Luis", "Pablo", "Pedro", "Jorge",
        ],
        last_names: &[
            "García", "Martínez", "López", "Rodríguez", "González", "Pérez", "Sánchez", "Gómez",
            "Martín", "Jiménez", "Ruiz", "Hernández", "Díaz", "Moreno", "Muñoz", "Romero",
        ],
        cities: &[
            ("Madrid", 28001), ("Barcelona", 08001), ("Valencia", 46001), ("Sevilla", 41001),
            ("Zaragoza", 50001), ("Málaga", 29001), ("Murcia", 30001), ("Palma de Mallorca", 07001),
        ],
        universities: &[
            "Complutense University of Madrid", "University of Barcelona",
            "Autonomous University of Madrid", "University of Valencia",
        ],
        employers: &[
            ("Santander", (30000, 60000)), ("BBVA", (32000, 62000)),
            ("Telefónica", (35000, 65000)), ("Iberdrola", (34000, 64000)),
            ("Inditex", (28000, 58000)),
        ],
    }
}

// ─── ITALIE ─────────────────────────────────────────────

fn italy_dictionaries() -> CountryDictionaries {
    CountryDictionaries {
        first_names: &[
            "Mario", "Giuseppe", "Antonio", "Francesco", "Luigi", "Paolo", "Pietro", "Marco",
            "Maria", "Anna", "Giuseppina", "Rosa", "Teresa", "Lucia", "Caterina", "Elena",
        ],
        last_names: &[
            "Rossi", "Russo", "Ferrari", "Esposito", "Bianchi", "Romano", "Colombo", "Ricci",
            "Marino", "Greco", "Bruno", "Gallo", "Conti", "De Luca", "Mancini", "Costa",
        ],
        cities: &[
            ("Roma", 00118), ("Milano", 20121), ("Napoli", 80100), ("Torino", 10100),
            ("Firenze", 50100), ("Venezia", 30100), ("Bologna", 40100), ("Genova", 16100),
        ],
        universities: &[
            "Sapienza University of Rome", "University of Milan",
            "University of Bologna", "University of Naples Federico II",
        ],
        employers: &[
            ("Eni", (35000, 65000)), ("Enel", (34000, 64000)),
            ("Intesa Sanpaolo", (32000, 62000)), ("Generali", (33000, 63000)),
            ("Pirelli", (36000, 66000)),
        ],
    }
}

// ─── ROYAUME-UNI ────────────────────────────────────────

fn uk_dictionaries() -> CountryDictionaries {
    CountryDictionaries {
        first_names: &[
            "James", "John", "Robert", "David", "William", "Richard", "Thomas", "Charles",
            "Mary", "Margaret", "Elizabeth", "Sarah", "Jane", "Helen", "Alice", "Emma",
        ],
        last_names: &[
            "Smith", "Jones", "Williams", "Taylor", "Brown", "Davies", "Evans", "Thomas",
            "Wilson", "Roberts", "Johnson", "Lewis", "Walker", "Robinson", "Wood", "Thompson",
        ],
        cities: &[
            ("London", 100000), ("Birmingham", 200000), ("Manchester", 300000),
            ("Glasgow", 400000), ("Liverpool", 500000), ("Edinburgh", 600000),
            ("Bristol", 700000), ("Leeds", 800000),
        ],
        universities: &[
            "University of Oxford", "University of Cambridge", "Imperial College London",
            "University of Edinburgh", "University of Manchester",
        ],
        employers: &[
            ("HSBC", (35000, 75000)), ("Barclays", (34000, 74000)),
            ("Unilever", (36000, 76000)), ("BP", (38000, 78000)),
            ("AstraZeneca", (39000, 79000)),
        ],
    }
}

// ─── MEXIQUE ────────────────────────────────────────────

fn mexico_dictionaries() -> CountryDictionaries {
    CountryDictionaries {
        first_names: &[
            "José", "Juan", "Miguel", "Luis", "Carlos", "Manuel", "Francisco", "Javier",
            "María", "Guadalupe", "Ana", "Rosa", "Teresa", "Martha", "Patricia", "Verónica",
        ],
        last_names: &[
            "Hernández", "García", "Martínez", "González", "López", "Rodríguez", "Pérez", "Sánchez",
            "Ramírez", "Torres", "Flores", "Rivera", "Morales", "Ortiz", "Cruz", "Reyes",
        ],
        cities: &[
            ("Ciudad de México", 01000), ("Guadalajara", 44100), ("Monterrey", 64000),
            ("Puebla", 72000), ("Toluca", 50000), ("Tijuana", 22000), ("León", 37000),
            ("Querétaro", 76000),
        ],
        universities: &[
            "UNAM", "IPN", "UAM", "Tecnológico de Monterrey", "Universidad de Guadalajara",
            "BUAP", "UAEMéx",
        ],
        employers: &[
            ("Cemex", (12000, 40000)), ("Grupo Bimbo", (10000, 35000)),
            ("América Móvil", (14000, 42000)), ("FEMSA", (13000, 38000)),
            ("Alfa", (12000, 36000)), ("Grupo Carso", (11000, 34000)),
        ],
    }
}

// ─── BRÉSIL ─────────────────────────────────────────────

fn brazil_dictionaries() -> CountryDictionaries {
    CountryDictionaries {
        first_names: &[
            "João", "José", "Antônio", "Carlos", "Francisco", "Paulo", "Pedro", "Lucas",
            "Maria", "Ana", "Francisca", "Antônia", "Adriana", "Juliana", "Fernanda", "Patrícia",
        ],
        last_names: &[
            "Silva", "Santos", "Souza", "Pereira", "Oliveira", "Carvalho", "Rodrigues", "Almeida",
            "Nascimento", "Lima", "Araújo", "Fernandes", "Costa", "Gomes", "Martins", "Rocha",
        ],
        cities: &[
            ("São Paulo", 01000), ("Rio de Janeiro", 20000), ("Brasília", 70000),
            ("Salvador", 40000), ("Fortaleza", 60000), ("Belo Horizonte", 30000),
            ("Manaus", 69000), ("Curitiba", 80000),
        ],
        universities: &[
            "USP", "UNICAMP", "UFRJ", "UNESP", "UFMG", "UFRGS", "UFSC", "PUC-SP",
        ],
        employers: &[
            ("Petrobras", (15000, 45000)), ("Vale", (14000, 42000)),
            ("Ambev", (12000, 38000)), ("Itaú Unibanco", (13000, 40000)),
            ("Bradesco", (12000, 38000)), ("Magazine Luiza", (10000, 35000)),
        ],
    }
}

// ─── ARGENTINE ──────────────────────────────────────────

fn argentina_dictionaries() -> CountryDictionaries {
    CountryDictionaries {
        first_names: &[
            "Juan", "Carlos", "José", "Luis", "Miguel", "Alberto", "Manuel", "Javier",
            "María", "Ana", "Laura", "Silvia", "Marta", "Cristina", "Paula", "Andrea",
        ],
        last_names: &[
            "González", "Rodríguez", "García", "López", "Martínez", "Fernández", "Pérez", "Sánchez",
            "Romero", "Sosa", "Alvarez", "Torres", "Díaz", "Ruiz", "Castro", "Gómez",
        ],
        cities: &[
            ("Buenos Aires", 1000), ("Córdoba", 5000), ("Rosario", 2000),
            ("Mendoza", 5500), ("La Plata", 1900), ("San Miguel de Tucumán", 4000),
            ("Mar del Plata", 7600), ("Salta", 4400),
        ],
        universities: &[
            "UBA", "UNC", "UNLP", "UTN", "Universidad de Belgrano", "Universidad de San Andrés",
        ],
        employers: &[
            ("YPF", (14000, 42000)), ("Techint", (13000, 40000)),
            ("Arcor", (12000, 38000)), ("Banco Galicia", (13000, 40000)),
            ("Mercado Libre", (15000, 45000)), ("Grupo Clarín", (11000, 35000)),
        ],
    }
}

// ─── COLOMBIE ───────────────────────────────────────────

fn colombia_dictionaries() -> CountryDictionaries {
    CountryDictionaries {
        first_names: &[
            "Juan", "Carlos", "José", "Luis", "Andrés", "David", "Jorge", "Daniel",
            "María", "Ana", "Luisa", "Catalina", "Paula", "Valentina", "Laura", "Camila",
        ],
        last_names: &[
            "Rodríguez", "García", "Martínez", "González", "López", "Fernández", "Pérez", "Sánchez",
            "Ramírez", "Torres", "Flores", "Rivera", "Morales", "Ortiz", "Cruz", "Reyes",
        ],
        cities: &[
            ("Bogotá", 11001), ("Medellín", 50001), ("Cali", 76001), ("Barranquilla", 80001),
            ("Cartagena", 13001), ("Cúcuta", 54001), ("Bucaramanga", 68001), ("Pereira", 66001),
        ],
        universities: &[
            "Universidad Nacional de Colombia", "Universidad de los Andes",
            "Universidad de Antioquia", "Universidad del Valle", "Pontificia Universidad Javeriana",
            "Universidad Industrial de Santander",
        ],
        employers: &[
            ("Ecopetrol", (12000, 38000)), ("Grupo Sura", (11000, 36000)),
            ("Bancolombia", (13000, 40000)), ("Nutresa", (10000, 35000)),
            ("Avianca", (12000, 37000)), ("Carvajal", (9000, 32000)),
        ],
    }
}
