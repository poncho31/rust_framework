// browser.rs
// Un navigateur TUI minimal en Rust, avec :
// - Une zone de recherche (Wikipedia via son API REST retournant du JSON)
//   qui récupère uniquement la définition (champ "extract")
// - Une zone URL pour naviguer directement sur un site (JSON attendu)
// - Un bouton pour lancer un nouveau terminal (nouvelle instance du programme)
// Note : Ce navigateur interroge l’API Wikipedia et affiche la définition extraite du JSON.

use cursive::{traits::*, CursiveExt};
use cursive::views::{Dialog, EditView, LinearLayout, ScrollView, TextView};
use cursive::Cursive;
use reqwest::blocking::get;
use serde_json::Value;
use std::process::Command;
use std::thread;

pub fn browser_run() {
    let mut siv = Cursive::default();
    // Couche principale
    siv.add_layer(main_ui());
    siv.run();
}

/// Construit l'interface principale regroupant la zone de recherche,
/// la zone URL et un bouton pour lancer un nouveau terminal.
fn main_ui() -> Dialog {
    let layout = LinearLayout::vertical()
        // Zone de recherche (utilise l'API Wikipedia pour obtenir du JSON)
        .child(
            Dialog::around(
                EditView::new()
                    .on_submit(|s, query| {
                        // Construit l'URL de l'API REST de Wikipedia pour obtenir le résumé
                        // Par exemple : https://fr.wikipedia.org/api/rest_v1/page/summary/Rust_(langage)
                        let url = format!("https://fr.wikipedia.org/api/rest_v1/page/summary/{}", query);
                        open_page(s, &url);
                    })
                    .with_name("search")
                    .fixed_width(50),
            )
            .title("Recherche Wikipedia (définition)")
            .padding_lrtb(1, 1, 1, 0),
        )
        // Zone URL pour navigation directe (fonctionne de la même façon)
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
        // Bouton pour lancer un nouveau terminal (nouvelle instance dans une nouvelle fenêtre)
        .child(
            Dialog::new().button("Nouveau Terminal", |s| {
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
        );
    Dialog::around(layout).title("Navigateur Rust (TUI)")
}

/// Ouvre une page dans une nouvelle couche en effectuant la requête dans un thread séparé.
fn open_page(s: &mut Cursive, url: &str) {
    let url_string = url.to_string();
    let cb_sink = s.cb_sink().clone();

    // Décharger la requête dans un thread pour ne pas bloquer l'interface
    thread::spawn(move || {
        let content = fetch_json(&url_string);
        // Mise à jour de l'UI via cb_sink
        cb_sink
            .send(Box::new(move |s: &mut Cursive| {
                s.add_layer(
                    Dialog::around(
                        ScrollView::new(TextView::new(content)).scrollable()
                    )
                    .title(&url_string)
                    .button("Retour", |s| {
                        s.pop_layer();
                    }),
                );
            }))
            .unwrap_or_else(|e| eprintln!("Erreur lors de l'envoi du callback : {:?}", e));
    });
}

/// Récupère le contenu JSON de l'URL donnée, extrait le champ "extract"
/// et le renvoie formaté.
pub fn fetch_json(url: &str) -> String {
    match get(url) {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Value>() {
                    Ok(json) => {
                        // Pour l'API Wikipedia, on cherche le champ "extract"
                        if let Some(def) = json.get("extract") {
                            def.as_str().unwrap_or("Définition introuvable").to_string()
                        } else {
                            "Aucune définition trouvée dans la réponse JSON.".to_string()
                        }
                    },
                    Err(e) => format!("Erreur lors de la lecture du JSON : {}", e),
                }
            } else {
                format!("Erreur HTTP: {}", resp.status())
            }
        },
        Err(e) => format!("Erreur de connexion: {}", e),
    }
}
