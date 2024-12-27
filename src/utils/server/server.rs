// server.rs
use tokio::task;
use crate::utils::env;
use crate::utils::server::web_server;
use crate::utils::server::web_view;
use actix_web::{web};
pub async fn run(
        routes: fn(&mut web::ServiceConfig),
        resources: fn(&mut web::ServiceConfig),
    ) -> std::io::Result<()> {

    // Vérifier le mode de lancement (webview ou serveur web)
    let app_mode = env::get("APP_MODE");

    // Lancer la fonction appropriée selon le mode
    if app_mode == "webview" {
        // Démarrer le serveur web de manière asynchrone en utilisant tokio::spawn_blocking
        task::spawn_blocking(move || {
            let _ = actix_rt::System::new().block_on(web_server::run(routes, resources));
        });

        // Exécuter WebView dans le thread principal + le serveru web
        web_view::run().expect("Erreur run web view");
    } else {
        // Exécuter uniquement le serveur web
        web_server::run(routes, resources).await?;
    }

    Ok(())
}
