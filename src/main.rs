#[macro_use]
extern crate lazy_static;
extern crate serde;

use actix_web::{App, HttpServer};
use actix_web::{get, http, web, HttpRequest, HttpResponse, Result};

use askama::Template;

mod config;

use crate::config::CONFIG;
use crate::config::Config;
use crate::config::LoginType;

#[derive(Template)]
#[template(path = "autoconfig.xml")]
pub struct AutoConfig {
    pub c: &'static Config,
}

#[derive(Template)]
#[template(path = "autodiscover-xml.html")]
pub struct AutoDiscoverXml {
    pub c: &'static Config,
}

#[derive(Template)]
#[template(path = "autodiscover-json.html")]
pub struct AutoDiscoverJson {
    pub c: &'static Config,
}

#[get("/.well-known/autoconfig/mail/config-v1.1.xml")]
pub async fn autoconfig(req: HttpRequest) -> Result<HttpResponse> {
    HttpResponse::Ok().content_type("text/xml").body(AutoConfig {
        c: &CONFIG
    }.render().expect("Failed to render template")).await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Reading configuration file...");
    println!("Autodiscover running for {}", &CONFIG.general.full_name);
    println!("Hello, world!");
    HttpServer::new(move || {
        App::new()
            .service(autoconfig)
    })
    .bind(&CONFIG.general.listening_address)?
        .run()
        .await
}
