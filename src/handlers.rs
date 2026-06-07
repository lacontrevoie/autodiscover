use actix_web::{http, web, HttpResponse, Result};
use askama::Template;
use std::collections::HashMap;

use crate::config::*;
use crate::errors::ServerError;
use crate::helpers::*;
use crate::structs::*;

pub async fn autoconfig(getdata: web::Query<HashMap<String, String>>) -> Result<HttpResponse, ServerError> {
    let mut tpl = AutoConfig { c: CONFIG.wait() }
        .render()
        .map_err(|_| ServerError::TemplateRenderFailAutoconf)?;
    // support thunderbird's special gimmick
    if let Some(addr) = getdata.get("emailaddress") {
        illegal_char_check(addr)?;
        tpl = tpl.replace("%EMAILADDRESS%", addr).replace(
            "%EMAILLOCALPART%",
            addr.split('@').collect::<Vec<&str>>()[0],
        );
    }

    Ok(HttpResponse::Ok().content_type("text/xml").body(tpl))
}

pub async fn autodiscover_json() -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().content_type("application/json").body(
        AutoDiscoverJson { c: CONFIG.wait() }
            .render()
            .map_err(|_| ServerError::TemplateRenderFailAdJson)?
    ))
}

pub async fn mobileconfig(getdata: web::Query<HashMap<String, String>>) -> Result<HttpResponse, ServerError> {
    let email = get_email_address(None, getdata.into_inner());

    Ok(HttpResponse::Ok()
        .content_type("application/x-apple-aspen-config; charset=utf-8")
        .insert_header((
            http::header::CONTENT_DISPOSITION,
            format!(
                "attachment; filename={}.mobileconfig",
                &CONFIG.wait().general.domain
            ),
        ))
        .body(
            MobileConfigXml {
                c: CONFIG.wait(),
                email: &email,
                uuid: &gen_uuid(),
                uuid_2: &gen_uuid(),
            }
            .render()
            .map_err(|_| ServerError::TemplateRenderFailMbc)?
        ))
}

pub async fn autodiscover_xml_get(
    getdata: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ServerError> {
    let schema = get_schema(None);
    let email = get_email_address(None, getdata.into_inner());

    Ok(HttpResponse::Ok().content_type("text/xml").body(
        AutoDiscoverXml {
            c: CONFIG.wait(),
            schema: &schema,
            email: &email,
        }
        .render()
        .map_err(|_| ServerError::TemplateRenderFailAdXmlGet)?
    ))
}

pub async fn autodiscover_xml_post(
    raw_post: web::Bytes,
    getdata: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, ServerError> {
    let xml_post = read_xml(raw_post).map_err(|_| ServerError::ReadXmlFail)?;

    let schema = get_schema(Some(xml_post.clone()));
    let email = get_email_address(Some(xml_post), getdata.into_inner());

    Ok(HttpResponse::Ok().content_type("text/xml").body(
        AutoDiscoverXml {
            c: CONFIG.wait(),
            schema: &schema,
            email: &email,
        }
        .render()
        .map_err(|_| ServerError::TemplateRenderFailAdXmlPost)?
    ))
}

