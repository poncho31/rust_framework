use actix_web::web;
use crate::models::_models::{User, NewUser}; // `crate` se réfère à la racine du projet (src)
use crate::schema::_schema::{users};          // `crate` se réfère à la racine du projet (src)
use crate::schema::_schema::users::dsl::*;    // Pour le DSL des tables Diesel

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use diesel::SqliteConnection;

use log::warn;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn paginate_users(pool: web::Data<DbPool>, page: Option<i64>, per_page: Option<i64>) -> Vec<User> {
    let mut conn = pool.get().expect("Couldn't get DB connection");

    // Valeurs par défaut : page = 1 et per_page = 10
    let page     = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);

    let offset = (page - 1) * per_page;

    users
        .order(id.desc())  // Trier par id dans l'ordre décroissant
        .limit(per_page)  // Limiter le nombre d'événements retournés
        .offset(offset)   // Décaler pour paginer
        .load::<User>(&mut conn)  // Charger les événements
        .expect("Error loading users")  // Retourner la liste des événements
}



pub fn insert_user(new_user: &NewUser, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
    match diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => {
            warn!("Erreur lors de l'insertion de l'utilisateur : {:?}", e);
            Err(e)
        }
    }
}
