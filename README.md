# File Integrity Monitoring
Ce projet en Rust surveille les opérations (modifications, créations, suppressions, déplacements, etc.) dans un répertoire, garantissant l'intégrité des fichiers. Les alertes sont affichées dans des pop-ups et enregistrées dans des logs. Chaque modification est enregistrée avec un "diff" et une copie du fichier est conservée pour comparaison.

## Utilisation :
Exécutez la commande suivante pour lancer le programme :

``
cargo run <dossier à surveiller>
``

Attention : Évitez d'utiliser ce projet pour surveiller des dossiers volumineux en raison de la copie complète des fichiers.

## Fonctionnalités à venir :
- Mise en place d'une interface de commande permettant d'afficher les logs d'un fichier spécifique et les modifications associées.
