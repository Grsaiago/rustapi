use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct ContactRouter {}

impl ContactRouter {
    pub fn setup_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/contact"))
            .route("/", web::get().to(Self::get_all_contacts));
    }

    async fn get_all_contacts() -> impl Responder {
        HttpResponse::Ok().body("<h1>Hello World</h1>")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = HttpServer::new(|| App::new().configure(ContactRouter::setup_routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    Ok(())
}
