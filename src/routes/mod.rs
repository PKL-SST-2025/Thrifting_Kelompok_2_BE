use actix_web::web;

pub mod auth;
pub mod products;
pub mod wishlist;
pub mod notifications;
pub mod settings;
pub mod stores;
pub mod support;
pub mod faq;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::scope())
        .service(products::scope())
        .service(wishlist::scope())
        .service(notifications::scope())
        .service(settings::scope())
        .service(stores::scope())
        .service(support::scope())
        .service(faq::scope());
}
