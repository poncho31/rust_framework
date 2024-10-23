use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use actix_web::{web, HttpResponse};
use diesel::r2d2::PooledConnection;
use log::warn;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_connection(pool: web::Data<DbPool>) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, HttpResponse> {
    match pool.get() {
        Ok(conn) => Ok(conn),
        Err(e) => {
            warn!("Impossible d'obtenir une connexion à la base de données : {:?}", e);
            Err(HttpResponse::InternalServerError().json("Erreur de connexion à la base de données"))
        }
    }
}
