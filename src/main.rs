use std::{env, fs, os::unix::fs::PermissionsExt, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(chemin) = args.get(1) else {
        eprintln!("Usage : {} <fichier.AppImage>", args[0]);
        std::process::exit(1);
    };
    if let Err(e) = installer(Path::new(chemin)) {
        eprintln!("Erreur : {e}");
        std::process::exit(1);
    }
}

fn installer(source: &Path) -> std::io::Result<()> {
    let dossier = dirs::home_dir()
        .expect("pas de dossier personnel")
        .join("Applications");
    fs::create_dir_all(&dossier)?;

    let destination = dossier.join(source.file_name().expect("nom de fichier invalide"));
    fs::copy(source, &destination)?;

    let mut droits = fs::metadata(&destination)?.permissions();
    droits.set_mode(0o755); // rwxr-xr-x
    fs::set_permissions(&destination, droits)?;

    println!("Installé : {}", destination.display());
    Ok(())
}
