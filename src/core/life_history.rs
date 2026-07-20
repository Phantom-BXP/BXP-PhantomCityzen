//! Historique de vie du citoyen : éducation, emploi, logement.

/// Études
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Education {
    pub degree: [u8; 64],
    pub institution: [u8; 64],
    pub graduation_year: u16,
}

/// Emploi
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Employment {
    pub employer: [u8; 64],
    pub position: [u8; 64],
    pub annual_salary: u32,
    pub start_year: u16,
}

/// Logement
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Housing {
    pub housing_type: u8,
    pub monthly_rent: u16,
    pub entry_year: u16,
}

// --- DICTIONNAIRES STATIQUES ---

/// 10 diplômes courants
pub const DEGREES: &[&str] = &[
    "Licence en Informatique",
    "Master en Cybersécurité",
    "BTS SIO",
    "DUT Informatique",
    "Ingénieur en Systèmes d'Information",
    "Licence en Mathématiques",
    "Master en Finance",
    "BTS Comptabilité",
    "DUT Gestion",
    "Licence en Droit",
];

/// 50 universités/écoles françaises
pub const UNIVERSITIES: &[&str] = &[
    "Université Paris-Saclay", "Sorbonne Université", "Université de Lyon",
    "Université de Toulouse", "Université de Montpellier", "Université de Bordeaux",
    "Université de Strasbourg", "Université de Lille", "Université de Rennes",
    "Université de Nantes", "Université de Grenoble", "Université de Nice",
    "Université de Tours", "Université de Poitiers", "Université de Metz",
    "Université de Nancy", "Université de Besançon", "Université de Dijon",
    "Université de Clermont-Ferrand", "Université de Limoges", "Université de Pau",
    "Université de Perpignan", "INSA Lyon", "INSA Toulouse", "INSA Rennes",
    "INSA Strasbourg", "CentraleSupélec", "Télécom Paris", "Télécom SudParis",
    "ENSTA Paris", "ENSAE Paris", "École Polytechnique", "ENS Lyon",
    "ENS Paris-Saclay", "ENS Rennes", "Université de Technologie de Compiègne",
    "Université de Technologie de Belfort-Montbéliard", "Université de Technologie de Troyes",
    "École Centrale de Lyon", "École Centrale de Nantes", "École Centrale de Marseille",
    "École des Mines de Paris", "École des Mines de Saint-Étienne", "École des Mines de Nancy",
    "École des Mines d'Alès",
];

/// 50 entreprises avec fourchettes de salaire (min, max)
pub const EMPLOYERS: &[(&str, (u32, u32))] = &[
    ("Sopra Steria", (35000, 55000)), ("Capgemini", (38000, 58000)),
    ("Accenture", (40000, 65000)), ("Atos", (36000, 56000)),
    ("IBM", (42000, 72000)), ("Microsoft", (45000, 80000)),
    ("Google France", (50000, 90000)), ("Amazon", (44000, 78000)),
    ("Oracle", (43000, 73000)), ("Salesforce", (46000, 82000)),
    ("SAP", (41000, 71000)), ("Dassault Systèmes", (40000, 70000)),
    ("Thales", (39000, 69000)), ("Airbus", (41000, 71000)),
    ("Safran", (39000, 69000)), ("EDF", (37000, 67000)),
    ("Engie", (36000, 66000)), ("SNCF", (34000, 64000)),
    ("RATP", (33000, 63000)), ("La Poste", (31000, 61000)),
    ("Orange", (35000, 65000)), ("Bouygues Telecom", (34000, 64000)),
    ("SFR", (33000, 63000)), ("Free", (32000, 62000)),
    ("BNP Paribas", (38000, 78000)), ("Crédit Agricole", (36000, 76000)),
    ("Société Générale", (37000, 77000)), ("HSBC", (39000, 79000)),
    ("LVMH", (35000, 75000)), ("L'Oréal", (34000, 74000)),
    ("TotalEnergies", (40000, 80000)), ("Schneider Electric", (37000, 77000)),
    ("Vinci", (35000, 75000)), ("Bouygues", (34000, 74000)),
    ("Eiffage", (33000, 73000)), ("Saint-Gobain", (36000, 76000)),
    ("Michelin", (34000, 74000)), ("Renault", (38000, 78000)),
    ("Stellantis", (37000, 77000)), ("Valeo", (35000, 75000)),
    ("Faurecia", (34000, 74000)), ("Air Liquide", (39000, 79000)),
    ("Lactalis", (32000, 72000)), ("Danone", (33000, 73000)),
    ("Carrefour", (30000, 70000)), ("Auchan", (29000, 69000)),
    ("Leclerc", (28000, 68000)), ("Intermarché", (27000, 67000)),
    ("Système U", (26000, 66000)), ("Aldi", (25000, 65000)),
];
