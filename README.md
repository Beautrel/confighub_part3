# ConfigHub - Partie 3

Ceci est le code pour la partie 3 du projet ConfigHub. Pour l'instant, le depot contient uniquement ma partie du travail : la mise en place du store partage (stub) et le systeme de broadcast asynchrone avec tokio.

## Comment lancer le projet

1. Ouvrez un terminal dans le dossier `confighub_part3`.
2. Tapez la commande : `cargo run`

## Remarques importantes pour la correction (Erreurs normales)

Etant donne que mon binome n'a pas encore integre sa partie (la boucle reseau TCP), vous allez remarquer que le programme s'arrete tout de suite apres l'execution. 

C'est un comportement normal et non un bug. Sans la boucle `loop` du TcpListener pour attendre les clients, la fonction main se termine directement.

Autres points documentes :
- Saturation du broadcast : Le channel est limite a 64 messages. Si cette limite est depassee plus tard dans le projet final, tokio renverra une erreur `RecvError::Lagged`. C'est la gestion prevue pour eviter les fuites memoire.
- Le ConfigStore actuel est un stub (faux store) pour simuler la partie 1. La methode `changes_since` renvoie tout le dictionnaire actuel au lieu d'un vrai historique des changements.