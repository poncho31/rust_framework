use actix_web::web;
use crate::models::_models::{Event, NewEvent}; // `crate` se réfère à la racine du projet (src)
use crate::schema::_schema::{events};          // `crate` se réfère à la racine du projet (src)
use crate::schema::_schema::events::dsl::*;    // Pour le DSL des tables Diesel

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use diesel::SqliteConnection;

use log::{warn};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn paginate_events(pool: web::Data<DbPool>, page: Option<i64>, per_page: Option<i64>) -> Vec<Event> {
    let mut conn = pool.get().expect("Couldn't get DB connection");

    // Valeurs par défaut : page = 1 et per_page = 10
    let page     = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);

    let offset = (page - 1) * per_page;

    events
        .order(id.desc())  // Trier par id dans l'ordre décroissant
        .limit(per_page)  // Limiter le nombre d'événements retournés
        .offset(offset)   // Décaler pour paginer
        .load::<Event>(&mut conn)  // Charger les événements
        .expect("Error loading events")  // Retourner la liste des événements
}



pub fn insert_event(new_event: &NewEvent, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
    match diesel::insert_into(events::table)
        .values(new_event)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => {
            warn!("Erreur lors de l'insertion de l'événement : {:?}", e);
            Err(e)
        }
    }
}
