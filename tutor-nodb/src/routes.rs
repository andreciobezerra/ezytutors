use actix_web::web::{self, ServiceConfig};

use crate::handlers::health_check_handler;

pub fn general_routes(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}
