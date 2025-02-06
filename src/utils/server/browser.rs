// terminal.rs

use cursive::{traits::*, CursiveExt};
use cursive::views::{Dialog, EditView, LinearLayout, ScrollView, TextView};
use cursive::Cursive;
use reqwest::blocking::get;
use serde_json::Value;
use std::process::Command;
use std::thread;

pub fn run_terminal_or_spawn() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    // Vérifie si l'argument "--terminal" est présent
    if args.len() > 1 && args[1] == "--terminal" {
        // Exécute uniquement le navigateur TUI
        terminal_run();
        return Ok(());
    }

    // Sinon, lance le navigateur dans un nouveau terminal
    let exe = std::env::current_exe()?;
    #[cfg(target_os = "windows")]
    {
        // Sur Windows, on utilise "cmd" et "start" pour ouvrir une nouvelle fenêtre.
        Command::new("cmd")
            .args(&["/C", "start", "", exe.to_str().unwrap(), "--terminal"])
            .spawn()
            .expect("Échec du lancement du navigateur dans un nouveau terminal");
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Sur Unix, on utilise "xterm". Vous pouvez adapter (ex: gnome-terminal, konsole, etc.)
        Command::new("xterm")
            .arg("-e")
            .arg(format!("{} --terminal", exe.to_str().unwrap()))
            .spawn()
            .expect("Échec du lancement du navigateur dans un nouveau terminal");
    }

    Ok(())
}


pub fn terminal_run() {
    let mut siv = Cursive::default();
    // Couche principale
    siv.add_layer(main_ui());
    siv.run();
}

/// Construit l'interface principale regroupant la zone de recherche,
/// la zone URL et un bouton pour lancer un nouveau terminal.
fn main_ui() -> Dialog {
    // Notre layout vertical
    let layout = LinearLayout::vertical()
        // 1) Zone de recherche (Wikipedia)
        .child(
            Dialog::around(
                EditView::new()
                    .on_submit(|s, query| {
                        let url = format!(
                            "https://fr.wikipedia.org/api/rest_v1/page/summary/{}",
                            query
                        );
                        open_page(s, &url);
                    })
                    .with_name("search")
                    .fixed_width(50),
            )
            .title("Recherche Wikipedia (définition)")
            .padding_lrtb(1, 1, 1, 0),
        )
        // 2) Zone URL pour navigation directe
        .child(
            Dialog::around(
                EditView::new()
                    .on_submit(|s, url| {
                        open_page(s, url);
                    })
                    .with_name("url")
                    .fixed_width(50),
            )
            .title("Navigation URL (JSON attendu)")
            .padding_lrtb(1, 1, 1, 0),
        )
        // 3) Bouton pour lancer un nouveau terminal
        .child(
            Dialog::new()
                .button("Nouveau Terminal", |_s| {
                    let exe = std::env::current_exe().expect("Impossible d'obtenir l'exécutable");
                    #[cfg(target_os = "windows")]
                    {
                        Command::new("cmd")
                            .args(&["/C", "start", "", exe.to_str().unwrap()])
                            .spawn()
                            .expect("Échec du lancement d'un nouveau terminal");
                    }
                    #[cfg(not(target_os = "windows"))]
                    {
                        Command::new("xterm")
                            .arg("-e")
                            .arg(exe.to_str().unwrap())
                            .spawn()
                            .expect("Échec du lancement d'un nouveau terminal");
                    }
                })
                .fixed_width(20),
        )
        // 4) **NOUVEAU** : Bouton pour redémarrer Cargo (cargo run)
        .child(
            Dialog::new()
                .button("Redémarrer Cargo", |_s| {
                    // Lance la commande `cargo run` (en tâche de fond)
                    #[cfg(target_os = "windows")]
                    {
                        let mut cmd = Command::new("cmd");
                        cmd.args(&["/C", "cargo", "run"])
                            .spawn()
                            .expect("Échec du redémarrage de cargo run");
                    }
                    #[cfg(not(target_os = "windows"))]
                    {
                        // Sur Unix, on lance directement `cargo run`.
                        Command::new("cargo")
                            .arg("run")
                            .spawn()
                            .expect("Échec du redémarrage de cargo run");
                    }
                })
                .fixed_width(20),
        );

    Dialog::around(layout).title("Navigateur Rust (TUI)")
}

/// Ouvre une page dans une nouvelle couche en effectuant la requête dans un thread séparé.
fn open_page(s: &mut Cursive, url: &str) {
    let url_string = url.to_string();
    let cb_sink = s.cb_sink().clone();

    thread::spawn(move || {
        let content = fetch_json(&url_string);
        // Mise à jour de l'UI via cb_sink
        if let Err(e) = cb_sink.send(Box::new(move |s: &mut Cursive| {
            s.add_layer(
                Dialog::around(ScrollView::new(TextView::new(content)).scrollable())
                    .title(&url_string)
                    .button("Retour", |s| {
                        s.pop_layer();
                    }),
            );
        })) {
            eprintln!("Erreur lors de l'envoi du callback : {:?}", e);
        }
    });
}

/// Récupère du JSON depuis l’URL, en extrait le champ "extract" si présent.
pub fn fetch_json(url: &str) -> String {
    match get(url) {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Value>() {
                    Ok(json) => {
                        if let Some(def) = json.get("extract") {
                            def.as_str().unwrap_or("Définition introuvable").to_string()
                        } else {
                            "Aucune définition trouvée dans la réponse JSON.".to_string()
                        }
                    }
                    Err(e) => format!("Erreur lors de la lecture du JSON : {}", e),
                }
            } else {
                format!("Erreur HTTP: {}", resp.status())
            }
        }
        Err(e) => format!("Erreur de connexion: {}", e),
    }
}
