# HelloKitKill.exe

> ⚠️ Projet à but éducatif uniquement. N'utilisez **jamais** ce code de manière malveillante.

**RustEduRansom** est un ransomware simplifié écrit en Rust, conçu pour illustrer les mécanismes fondamentaux des ransomwares modernes dans un cadre pédagogique (CTF, formation SOC, blue team).

## Fonctionnalités

- Chiffrement de fichiers avec AES-256 (mode CBC)
- Génération de clé aléatoire
- Écriture d’une note de rançon (`README_RESTORE.txt`)
- Ciblage d’un répertoire spécifique

## Usage

> 🧪 À exécuter uniquement dans une VM isolée ou un environnement sécurisé.


```bash
cargo build --release
./target/release/hellokitkill --help
```

## Avertissement

Ce code est strictement éducatif. L’utilisation de ce type de programme sur des systèmes réels sans autorisation est illégale.