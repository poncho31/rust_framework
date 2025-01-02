use glob::glob;
use std::fs;
use std::path::{Path};

fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_dir = Path::new("target").join(&profile); // Répertoire cible général

    // Définitions des actions
    let patterns = vec![
        ("resources/**/*",        Action::CopyFiles("resources".into())), // Copier fichiers et sous-dossiers
        ("database/sqlite/",      Action::CreateDir),                   // Créer uniquement un dossier
        ("storage/app/test.json", Action::CopyFile),              // Copier un fichier spécifique
    ];

    for (pattern, action) in patterns {
        match action {
            Action::CopyFiles(base_dir) => copy_files(pattern, &target_dir.join(&base_dir), &base_dir),
            Action::CreateDir => create_dir(pattern, &target_dir),
            Action::CopyFile => copy_file(pattern, &target_dir),
        }
    }

    println!("All files and directories processed successfully!");
}

// Enum pour définir les actions
enum Action {
    CopyFiles(String), // Copier fichiers avec chemin relatif
    CreateDir,         // Créer uniquement un dossier
    CopyFile,          // Copier un fichier spécifique
}

// Fonction pour copier des fichiers et dossiers
fn copy_files(pattern: &str, target_dir: &Path, base_dir: &str) {
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let relative_path = path.strip_prefix(base_dir).unwrap_or(&path); // Chemin relatif
                let dest_path = target_dir.join(relative_path); // Chemin cible

                // Créer les dossiers nécessaires
                if let Some(parent) = dest_path.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent).unwrap();
                        println!("Directory {:?} created.", parent);
                    }
                }

                // Copier les fichiers
                if path.is_file() {
                    if !dest_path.exists() {
                        fs::copy(&path, &dest_path).unwrap();
                        println!("File {:?} copied to {:?}.", path, dest_path);
                    } else {
                        println!("File {:?} already exists, skipping.", dest_path);
                    }
                }
            }
            Err(e) => println!("Error reading file: {:?}", e),
        }
    }
}

// Fonction pour créer un dossier
fn create_dir(pattern: &str, target_dir: &Path) {
    let src_dir = Path::new(pattern.trim_end_matches('/')); // Supprime le `/` final
    let dest_dir = target_dir.join(src_dir);

    if !dest_dir.exists() {
        fs::create_dir_all(&dest_dir).unwrap();
        println!("Directory {:?} created.", dest_dir);
    }
}

// Fonction pour copier un fichier spécifique
fn copy_file(pattern: &str, target_dir: &Path) {
    let src_file = Path::new(pattern);
    let dest_file = target_dir.join(src_file);

    // Créer les dossiers nécessaires
    if let Some(parent) = dest_file.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).unwrap();
            println!("Directory {:?} created.", parent);
        }
    }

    // Copier le fichier
    if src_file.is_file() {
        if !dest_file.exists() {
            fs::copy(&src_file, &dest_file).unwrap();
            println!("File {:?} copied to {:?}.", src_file, dest_file);
        } else {
            println!("File {:?} already exists, skipping.", dest_file);
        }
    } else {
        println!("Source file {:?} does not exist.", src_file);
    }
}
