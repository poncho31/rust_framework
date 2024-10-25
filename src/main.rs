// CMD :
//      install :
//                rust               : curl https://sh.rustup.rs -sSf | sh
//                cargo watch        : cargo install cargo-watch
//                npm                : npm i | npm run build
//                diesel ORM install : cargo install diesel_cli --no-default-features --features sqlite-bundled
//                diesel config      : diesel setup
//      diesel migration run
//      cargo update
//      cargo watch -x run
//      cargo watch -x "run" --poll (pour vérifier les fichiers à intervalles réguliers au lieu de se baser sur les événements du système de fichiers)
//      cargo watch -x "run" --why --ignore "db.sqlite-journal" (sinon le serveur redémarre dès qu'il y a un insert en DB)


//MODULES
    mod controllers;
    mod schema;
    mod models;
    mod repository;
    mod database;
    mod utils;

// CRATE CONTROLLERS
// Imports des modules internes (ton propre code)
use crate::controllers::_event_controller::{list_events, add_event};
use crate::controllers::_user_controller::{list_users, add_user};

// Imports externes
use std::io::Write;
use actix_files as fs;
use actix_web::{web, App, HttpServer, middleware};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use dotenv::dotenv;
use tera::Tera;
use log::{info, warn};
use env_logger::Builder;
use std::process::Command;
use std::sync::mpsc;
use std::thread;

// Pour gérer les WebView avec `wry`
use tao::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{WindowBuilder, Icon}};
use wry::WebViewBuilder;
use std::path::Path;
use tao::platform::windows::IconExtWindows;
use tokio::task;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser l'environnement et le logger
    dotenv().ok();
    Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .init();

    // Vérifier le mode de lancement (webview ou serveur web)
    let app_mode = utils::env::get("APP_MODE");

    // Launch appropriate function based on mode
    if app_mode == "webview" {
        // Start the web server asynchronously using tokio::spawn_blocking
        task::spawn_blocking(|| {
            let _ = actix_rt::System::new().block_on(run_web_mode());
        });

        // Run the WebView in the main thread
        run_webview_mode();
    } else {
        // Run only the web server
        run_web_mode().await?;
    }

    Ok(())
}


fn run_webview_mode() {
    // Initialize the tao event loop
    let event_loop = EventLoop::new();

    // Create a window with tao's WindowBuilder
    let window = WindowBuilder::new()
        .with_title("Event Manager")
        .with_inner_size(tao::dpi::LogicalSize::new(1000.0, 600.0)) // Sets the initial window size
        .with_resizable(true)                                  // Allows the window to be resizable
        .with_decorations(true)                                // Enables window decorations (title bar, close button)
        .with_always_on_top(true)                             // Keeps window on top of other windows
        .with_transparent(false)                               // Sets the window background to be transparent
        .with_fullscreen(None)                                 // Enables fullscreen mode (use options for specific display)
        .with_maximized(false)                                 // Opens the window in maximized state
        .with_visible(true)                                    // Controls the initial visibility of the window
        .with_window_icon(load_icon())                         // Sets a custom window icon
        .with_transparent(true)                                // Makes window background transparent
        .build(&event_loop)
        .expect("Failed to create window");

    // Initialize the WebView attached to the tao window
    let _webview = WebViewBuilder::new()
        .with_url("http://127.0.0.1:8082")  // Set your local server URL
        .build(&window)  // Attach to the tao window
        .expect("Failed to build WebView");

    // Run the event loop to handle window events
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested, ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

fn load_icon() -> Option<Icon> {
    let icon_path = Path::new("resources/images/icons/favicon.ico");
    Icon::from_path(icon_path, None).ok()
}

// Fonction pour lancer en mode serveur web normal
async fn run_web_mode() -> std::io::Result<()> {
    info!("Lancement en mode Serveur Web");

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(|cfg| configure_app(cfg, template_engine("tera")))
            .configure(resources)
            .configure(routes)
    })
        .workers(1)
        .bind("127.0.0.1:8082")?
        .run();

    let address = "localhost:81";
    info!("Serveur Proxy Nginx en cours de démarrage à l'adresse : {}", address);
    start_proxy_server();

    if let Err(e) = server.await {
        eprintln!("Erreur lors de l'exécution du serveur Actix-Web: {}", e);
    }

    Ok(())
}

fn start_proxy_server() {
    let proxy_webserver_name = utils::env::get("PROXY_WEB_SERVER_NAME");
    let delete_service = utils::env::get("PROXY_WEB_SERVER_CMD_DELETE_SERVICE");
    let create_service = utils::env::get("PROXY_WEB_SERVER_CMD_CREATE_SERVICE");
    let start_service = utils::env::get("PROXY_WEB_SERVER_CMD_START_SERVICE");
    info!("Proxy server {} en cours de démarrage...", proxy_webserver_name);


    // Supprimer le service existant (si nécessaire)
    if let Err(e) = Command::new("cmd")
        .args(["/C", &delete_service])
        .output()
    {
        eprintln!("Erreur lors de la suppression du service Nginx : {}", e);
    } else {
        info!("Service Nginx supprimé avec succès (si existant).");
    }

    // Créer le service Nginx
    if let Err(e) = Command::new("cmd")
        .args(["/C", &create_service])
        .output()
    {
        eprintln!("Erreur lors de la création du service Nginx : {}", e);
    } else {
        info!("Service Nginx créé avec succès.");
    }

    // Démarrer le service Nginx
    if let Err(e) = Command::new("cmd")
        .args(["/C", &start_service])
        .output()
    {
        eprintln!("Erreur lors du démarrage du service Nginx : {}", e);
    } else {
        info!("Service Nginx démarré avec succès.");
    }
}



fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // EVENTS
        .service(add_event)
        .service(list_events)

        // USERS
        .service(list_users)
        .service(add_user)
    ;
}

fn resources(cfg: &mut web::ServiceConfig){
    cfg
        // JS
        .service(fs::Files::new("/resources/js", "./resources/js").show_files_listing())

        // CSS
        .service(fs::Files::new("/resources/css", "./resources/css").show_files_listing())

        // RESOURCE IMAGE .ico
        .route("/favicon.ico", web::get().to(|| async {
            fs::NamedFile::open_async("./resources/images/icons/favicon.ico").await.unwrap()
        }));
}

fn configure_app(cfg: &mut web::ServiceConfig, tera: Tera) {
    cfg
        // Pool de connexions
        .app_data(web::Data::new(database::establish_connection_pool()))

        // Moteur de templates
        .app_data(web::Data::new(tera));
}

fn template_engine(name: &str) -> Tera {
    if name == "tera" {
        match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/views/**/*")) {
            Ok(t) => {
                info!("Moteur template Tera initialisé avec succès.");
                t.clone()  // Retourner l'objet `Tera` en cas de succès
            }
            Err(e) => {
                warn!("Erreur lors de l'initialisation du Moteur template Tera : {:?}", e);
                std::process::exit(1);
            }
        }
    } else {
        warn!("Aucun moteur de template trouvé pour: {}", name);
        std::process::exit(1);
    }
}
