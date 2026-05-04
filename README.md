# ConfigHub - Partie 3 (Serveur de souscription asynchrone)

Ce dépôt contient la première phase de la **Partie 3** du projet ConfigHub.
Le travail actuel se concentre sur les fondations asynchrones et la gestion de la mémoire partagée.

Ont été implémentés :
- **Le Stub du ConfigStore** (`src/store.rs`) : Simulation de la Partie 1 avec `Arc<RwLock<ConfigStore>>` permettant des accès concurrents sécurisés.
- **Le Système de Broadcast** (`src/pubsub.rs`) : Implémentation du pattern Pub/Sub via `tokio::sync::broadcast` pour distribuer les mises à jour aux abonnés.

---

## 🚀 Comment lancer le projet chez vous

### Prérequis
- Avoir la chaîne d'outils **Rust** installée (`rustc` et `cargo`).
- Avoir une **connexion internet active** lors du premier lancement (pour télécharger la bibliothèque asynchrone `tokio` et ses dépendances).

### Exécution
1. Ouvrez un terminal et placez-vous dans le dossier racine du projet (`confighub_part3`).
2. Lancez simplement la commande suivante :
   ```bash
   cargo run
   ```
3. Si tout fonctionne correctement, Cargo va compiler le projet et vous devriez obtenir la sortie suivante :
   ```text
   Lancement du serveur (Partie 3)
   Pret pour gerer les sockets TCP !
   ```

---

## ⚠️ Erreurs connues et comportements documentés

Pour respecter les exigences du projet, voici la documentation des comportements spécifiques et des "erreurs" normales liées à cette phase de développement :

### 1. Le programme se termine immédiatement après le lancement
* **Symptôme :** Le programme affiche le message de lancement puis rend la main au terminal immédiatement.
* **Explication (Non pénalisable) :** C'est tout à fait normal. Cette partie de l'équipe était chargée de la mise en place du Store et du système de Broadcast. La boucle réseau asynchrone (`TcpListener::bind` et `loop { accept() }`) sera intégrée par la suite. Sans cette boucle infinie, la fonction `main` s'exécute jusqu'au bout et se termine proprement sans erreur.

### 2. Le fonctionnement simplifié de `changes_since`
* **Symptôme :** L'appel à `store.changes_since(version)` renvoie la totalité de la configuration actuelle, et non uniquement le différentiel depuis la `version`.
* **Explication :** La Partie 1 étant développée par un autre groupe, nous utilisons un "Stub" (un simulacre). Cette implémentation simplifiée est volontaire et suffit amplement pour valider le fonctionnement de la Partie 3 en attendant l'intégration finale.

### 3. Gestion du Lagging (Saturation du Broadcast)
* **Comportement prévu :** Le canal de diffusion (`tokio::sync::broadcast`) est configuré avec une capacité fixe de **64 messages**. 
* **Explication :** Si, lors des tests finaux, un client a une connexion trop lente et n'arrive pas à consommer ses notifications assez vite, Tokio supprimera les vieux messages pour éviter une fuite de mémoire (Out Of Memory). Le client recevra alors une erreur `RecvError::Lagged`. C'est un mécanisme de sécurité intentionnel dicté par la conception asynchrone.