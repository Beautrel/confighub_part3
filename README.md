# Partie 3 — Serveur de souscription async (ConfigHub)

## Description

Cette partie implémente le serveur TCP asynchrone de ConfigHub.
Les clients peuvent s'abonner à des clés de configuration et recevoir
les mises à jour en temps réel sans redémarrage.

## Fonctionnement

1. Un client se connecte au serveur TCP sur le port 7878
2. Il envoie `SUBSCRIBE namespace.key` pour s'abonner à une clé
3. Il reste connecté et attend les mises à jour
4. Quand une valeur change via `SET namespace.key=value`, le serveur pousse automatiquement `UPDATE namespace.key=value` à tous les abonnés

## Commandes supportées

| Commande | Format | Description |
|----------|--------|-------------|
| SUBSCRIBE | `SUBSCRIBE namespace.key` | S'abonner à une clé |
| SET | `SET namespace.key=value` | Modifier une valeur |

## Technologies utilisées

- **Rust** — langage principal
- **tokio** — runtime async pour les connexions TCP
- **tokio::sync::broadcast** — diffusion des mises à jour aux abonnés
- **Arc<RwLock<ConfigStore>>** — partage thread-safe du store

## Structure du projet
src/ ├── main.rs — point d'entrée du serveur ├── server.rs — logique TCP et gestion des commandes └── store.rs — stub du ConfigStore (Partie 1)
examples/ ├── client.rs — client de test pour SUBSCRIBE └── mises à jour.rs — client de test pour SET

## Lancer le projet

**Démarrer le serveur :**
```bash
cargo run
```

**Dans un deuxième terminal — s'abonner à une clé :**
```bash
cargo run --example client
```

**Dans un troisième terminal — envoyer une mise à jour :**
```bash
cargo run --example updates
```

## Dépendances

La Partie 1 (ConfigStore) est simulée dans `store.rs`.
Ce module sera remplacé par l'implémentation réelle de la Partie 1
lors de l'assemblage final du projet.

## Auteurs

- [Beautrel TSAWO] — serveur TCP, gestion SUBSCRIBE/SET
- [Fils Ngock] — stub ConfigStore, système broadcast