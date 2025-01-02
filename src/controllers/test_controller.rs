use actix_web::{HttpResponse, web};
use crate::database::DbPool;
use crate::repository::event_repository;
use crate::utils::template_engine::template::{generate_html, html_footer, html_navbar, html_section};
use crate::view::event_table::{EventItem};

pub async fn test_inject_object_in_view(pool: web::Data<DbPool>) -> HttpResponse {
    // Récupération des événements depuis la base de données
    let all_events = event_repository::paginate_events(pool, None, None);
    println!("{:#?}", "ALL TEST");
    println!("{:#?}", all_events);

    // Génération du HTML avec les données paginées
    let html_output = generate_html::<_, EventItem>(
        all_events,
        "tera",
        html_navbar(),
        html_section(),
        html_footer(),
        "TEST PARAM",
    );

    // Retourner le HTML généré dans la réponse HTTP
    HttpResponse::Ok().content_type("text/html").body(html_output)
}



