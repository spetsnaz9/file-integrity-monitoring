# File Integrity Monitoring

Ce projet fait en rust vise à garantir l'intégrité des fichiers dans un répertoire donné en surveillant les opérations telles que les modifications, les créations, les suppressions, les déplacements, etc.

Pour chaque alerte générée, ce programme affiche une pop-up contenant les détails de l'événement et les enregistre également dans la sortie standard.

Toutes les alertes liées à un fichier sont enregistrées dans des logs.

Historique des modifications, chaque modification est représentée par un "diff", et une copie du fichier est faite pour la comparer a la nouvelle version de celui-ci.


## Utilisation :
Exécutez la commande suivante pour lancer le programme :

``
cargo run <dossier à surveiller>
``

Un dossier nommé "save" est inclus pour stocker les logs des fichiers ainsi que les informations sur les modifications qui leur sont associées.

## Fonctionnalités à venir :
- Mise en place d'une interface de commande permettant d'afficher les logs d'un fichier spécifique et les modifications associées.
