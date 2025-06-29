use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(hello));
    cfg.route("/chat", web::post().to(chat));
}
