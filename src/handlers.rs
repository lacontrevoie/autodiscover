use actix_web::{get, post, web, http, HttpResponse, Result};
use askama::Template;
use std::collections::HashMap;

use crate::config::*;
use crate::structs::*;
use crate::helpers::*;

pub async fn autoconfig(getdata: web::Query<HashMap<String, String>>)
    -> Result<HttpResponse> {
        let mut tpl = 
            AutoConfig { c: &CONFIG }
            .render()
            .expect("Failed to render template");
        // support thunderbird's special gimmick
        if let Some(addr) = getdata.get("emailaddress") {
            tpl = tpl.replace("%EMAILADDRESS%", addr)
                .replace("%EMAILLOCALPART%", addr.split('@').collect::<Vec<&str>>()[0]);
        }

    HttpResponse::Ok()
        .content_type("text/xml")
        .body(tpl)
        .await
}

#[get("/autodiscover/autodiscover.json")]
pub async fn autodiscover_json() 
-> Result<HttpResponse> {
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
    let email = get_email_address(None, getdata.into_inner());

    HttpResponse::Ok()
        .content_type("application/x-apple-aspen-config; charset=utf-8")
        .header(http::header::CONTENT_DISPOSITION, format!("attachment; filename={}.mobileconfig", &CONFIG.general.domain))
        .body(
            MobileConfigXml {
                c: &CONFIG,
                email: &email,
                uuid: &gen_uuid(),
                uuid_2: &gen_uuid(),
            }
            .render()
            .expect("Failed to render template"),
        )
        .await
}

#[get("/autodiscover/autodiscover.xml")]
pub async fn autodiscover_xml_get(
    getdata: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
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

#[post("/autodiscover/autodiscover.xml")]
pub async fn autodiscover_xml_post(
    raw_post: web::Bytes,
    getdata: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    let xml_post = read_xml(raw_post);

    let schema = get_schema(Some(xml_post.clone()));
    let email = get_email_address(Some(xml_post), getdata.into_inner());

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
