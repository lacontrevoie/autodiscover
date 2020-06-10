use actix_web::{get, post, web, http, HttpResponse, Result};
use askama::Template;
use std::collections::HashMap;

use crate::config::*;
use crate::structs::*;
use crate::helpers::*;

#[get("/mail/config-v1.1.xml")]
pub async fn autoconfig() -> Result<HttpResponse> {
    debug_mode("autoconfig", None, None);
    HttpResponse::Ok()
        .content_type("text/xml")
        .body(
            AutoConfig { c: &CONFIG }
            .render()
            .expect("Failed to render template"),
        )
        .await
}

#[get("/Autodiscover/Autodiscover.json")]
pub async fn autodiscover_json() 
-> Result<HttpResponse> {
    debug_mode("autodiscover_json", None, None);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(
            AutoDiscoverJson {
                c: &CONFIG,
            }
            .render()
            .expect("Failed to render template"),
        )
        .await
}

#[get("/email/mobileconfig")]
pub async fn mobileconfig(
    getdata: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    debug_mode("autodiscover_json", Some(getdata.clone()), None);
    let email = get_email_address(None, getdata.into_inner());

    HttpResponse::Ok()
        .content_type("application/x-apple-aspen-config; charset=utf-8")
        .header(http::header::CONTENT_DISPOSITION, format!("attachment; filename={}.mobileconfig", &CONFIG.general.domain))
        .body(
            MobileConfigXml {
                c: &CONFIG,
                email: &email,
            }
            .render()
            .expect("Failed to render template"),
        )
        .await
}

#[get("/Autodiscover/Autodiscover.xml")]
pub async fn autodiscover_xml_get(
    getdata: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    debug_mode("autodiscover_xml_get", Some(getdata.clone()), None);
    let schema = get_schema(None);
    let email = get_email_address(None, getdata.into_inner());

    HttpResponse::Ok()
        .content_type("text/xml")
        .body(
            AutoDiscoverXml {
                c: &CONFIG,
                schema: &schema,
                email: &email,
            }
            .render()
            .expect("Failed to render template"),
        )
        .await
}

#[post("/Autodiscover/Autodiscover.xml")]
pub async fn autodiscover_xml_post(
    postdata: web::Form<AutoDiscoverRequest>,
    getdata: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    debug_mode("autodiscover_xml_post", Some(getdata.clone()), Some(postdata.clone()));
    let schema = get_schema(Some(postdata.clone()));
    let email = get_email_address(Some(postdata.into_inner()), getdata.into_inner());

    HttpResponse::Ok()
        .content_type("text/xml")
        .body(
            AutoDiscoverXml {
                c: &CONFIG,
                schema: &schema,
                email: &email,
            }
            .render()
            .expect("Failed to render template"),
        )
        .await
}

