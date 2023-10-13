use std::env;



pub fn parser() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Utilisation: cargo run <nom_de_dossier>");
        std::process::exit(1);
    }

    // Récupérer le nom de fichier à partir du premier argument
    return args[1].clone();
}
