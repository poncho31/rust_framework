use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use actix_web::{web, HttpResponse};
use diesel::r2d2::PooledConnection;
use log::{info, warn};

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection_pool() -> crate::DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");
    info!("Initialisation du pool de connexions à la base de données...");
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}

pub fn get_connection(pool: web::Data<DbPool>) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, HttpResponse> {
    match pool.get() {
        Ok(conn) => Ok(conn),
        Err(e) => {
            warn!("Impossible d'obtenir une connexion à la base de données : {:?}", e);
            Err(HttpResponse::InternalServerError().json("Erreur de connexion à la base de données"))
        }
    }
}