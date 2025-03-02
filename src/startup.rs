use std::net::TcpListener;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}