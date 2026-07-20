# BXP-PhantomCitizen

**Moteur de génération d'identités numériques souveraines.**

---

## État actuel

⚙️ **Développement en cours** — La première brique (générateur déterministe avec dictionnaires statiques) est posée. Les prochaines étapes intègrent les données INSEE et le MRZ.

---

## Objectif

BXP-PhantomCitizen est un outil de résistance numérique. Il permet de générer des identités cohérentes, déterministes et statistiquement crédibles, sans base de données ni dépendances externes.

- **Léger** : Binaire unique < 2 Mo, mémoire < 5 Mo.
- **Portable** : `no_std + alloc`, compatible avec les environnements contraints.
- **Déterministe** : Une graine = une identité, partout, toujours.
- **Modulaire** : Conçu pour s'interfacer avec d'autres outils de la Brigade X Phantom (SecretSonar, etc.).

---

## Structure du dépôt

```

src/
├── core/
│   ├── alias_table.rs   # Alias Method (tirages pondérés O(1))
│   ├── citizen.rs       # Structure et générateur d'identité
│   ├── prng.rs          # SplitMix64 (PRNG déterministe)
│   └── mod.rs
├── lib.rs
└── main.rs              # Point d'entrée (tests)

```

---

## Compilation

```bash
cargo build --features minimal
```

---

Avertissement

Ce projet est en phase de conception et de développement. Il n'est pas destiné à un usage illégal. Il est pensé comme un outil de recherche et de résistance face aux dérives du contrôle numérique.
