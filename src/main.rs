use actix_web::{post, web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use bcrypt::hash;
use serde::Deserialize;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::{NewUser};
use diesel::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod models;
mod schema;

#[post("/register")]
async fn register(user_data: web::Json<RegisterData>, pool: web::Data<DbPool>) -> HttpResponse {
    use schema::users;

    let mut conn = pool.get().expect("Couldn't get DB connection");

    let new_user = NewUser {
        username: &user_data.username,
        email: &user_data.email,
        password_hash: &hash(&user_data.password, 4).unwrap(),
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)  // Ajout de `mut` ici
    {
        Ok(_) => HttpResponse::Ok().json("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to register user"),
    }
}

#[derive(Deserialize)]
struct RegisterData {
    username: String,
    email: String,
    password: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(establish_connection_pool()))
            .service(register)
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}

fn establish_connection_pool() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
