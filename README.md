# File Integrity Monitoring

Ce projet fait en rust vise à garantir l'intégrité des fichiers dans un répertoire donné en surveillant les opérations telles que les modifications, les créations, les suppressions, les déplacements, etc.

Pour chaque alerte générée, ce programme affiche une pop-up contenant les détails de l'événement et les enregistre également dans la sortie standard.

À l'initialisation, le projet effectue une analyse récursive du dossier spécifié et crée un log s'il constate qu'un fichier n'a jamais été analysé. Ensuite, chaque événement ultérieur lié à un fichier est enregistré dans ce log.

## Utilisation :
Exécutez la commande suivante pour lancer le programme :

``
cargo run <dossier à surveiller>
``

Un dossier nommé "save" est inclus pour stocker les logs des fichiers ainsi que les informations sur les modifications qui leur sont associées.

## Fonctionnalités à venir :
- Historique des modifications, chaque modification est représentée par un "diff", et une copie du fichier est faite pour la comparer a la nouvelle version de celui-ci.
- Affichage d'un historique pour chaque modification.
- Mise en place d'une interface de commande permettant d'afficher les logs d'un fichier spécifique et les modifications associées.
