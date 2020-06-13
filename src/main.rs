#[macro_use]
extern crate lazy_static;
extern crate serde;

mod config;
mod structs;
mod handlers;
mod helpers;

use actix_web::{App, HttpServer};

use crate::config::*;
use crate::handlers::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    println!("Reading configuration file...");
    println!("Autodiscover running for {}", &CONFIG.general.full_name);
    
    HttpServer::new(move || App::new()
        .service(autoconfig)
        .service(autodiscover_xml_get)
        .service(autodiscover_xml_post)
        .service(autodiscover_json)
        .service(mobileconfig)
    )
        .bind(&CONFIG.general.listening_address)?
        .run()
        .await
}
