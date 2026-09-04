use actix_web::{web, App};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/accounts")
       .route(web::post().to(handlers::create_account))
       .route(web::get().to(handlers::get_accounts)))
       .service(web::resource("/accounts/{id}")
       .route(web::get().to(handlers::get_account))
       .route(web::put().to(handlers::update_account))
       .route(web::delete().to(handlers::delete_account)));
}