use crate::api::users::login::login;
use crate::api::users::registration::register;
use crate::api::users::user::current_user;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .route("/api/users", web::post().to(register))
            .route("/api/users/login", web::post().to(login))
            .route("/api/user", web::get().to(current_user))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
