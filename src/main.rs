use actix_web::{App, HttpServer};
mod routes;
use crate::routes::contact_router::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = HttpServer::new(|| App::new().configure(ContactRouter::setup_routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    Ok(())
}
