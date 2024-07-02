use actix_web::{web::Data, HttpResponse, Responder};

use crate::state::AppState;

pub async fn health_check_handler(app_state: Data<AppState>) -> impl Responder {
    let mut visitor_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {}", &app_state.health_check_response, visitor_count);

    *visitor_count += 1;

    HttpResponse::Ok().json(response)
}
