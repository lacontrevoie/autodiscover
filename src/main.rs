#[macro_use]
extern crate lazy_static;
extern crate serde;

mod config;
mod handlers;
mod helpers;
mod structs;

use actix_web::{web, App, HttpServer};

use crate::config::*;
use crate::handlers::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Reading configuration file...");
    println!("Autodiscover running for {}", &CONFIG.general.full_name);

    HttpServer::new(move || {
        App::new()
            .route("/mail/config-v1.1.xml", web::to(autoconfig))
            .route(
                "/.well-known/autoconfig/mail/config-v1.1.xml",
                web::to(autoconfig),
            )
            .route(
                "/autodiscover/autodiscover.json",
                web::to(autodiscover_json),
            )
            .route(
                "/Autodiscover/Autodiscover.json",
                web::to(autodiscover_json),
            )
            .route(
                "/autodiscover/autodiscover.xml",
                web::get().to(autodiscover_xml_get),
            )
            .route(
                "/Autodiscover/Autodiscover.xml",
                web::get().to(autodiscover_xml_get),
            )
            .route(
                "/autodiscover/autodiscover.xml",
                web::post().to(autodiscover_xml_post),
            )
            .route(
                "/Autodiscover/Autodiscover.xml",
                web::post().to(autodiscover_xml_post),
            )
            .route("/email/mobileconfig", web::to(mobileconfig))
    })
    .bind(&CONFIG.general.listening_address)?
    .run()
    .await
}
