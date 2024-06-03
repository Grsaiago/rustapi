use actix_web::{web, HttpResponse, Responder};

pub struct ContactRouter {}

impl ContactRouter {
    pub fn setup_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/contact"))
            .route("/", web::get().to(Self::get_all_contacts));
    }

    async fn get_all_contacts() -> impl Responder {
        HttpResponse::Ok().body("<h1>Hello World</h1>")
    }
}
