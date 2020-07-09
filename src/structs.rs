use askama::Template;
use serde::{Deserialize, Serialize};

use crate::config::*;

#[derive(Template)]
#[template(path = "autoconfig.xml")]
pub struct AutoConfig {
    pub c: &'static Config,
}

#[derive(Template)]
#[template(path = "email.mobileconfig.xml")]
pub struct MobileConfigXml<'a> {
    pub c: &'static Config,
    pub email: &'a str,
    pub uuid: &'a str,
    pub uuid_2: &'a str,
}

#[derive(Template)]
#[template(path = "autodiscover-xml.html")]
pub struct AutoDiscoverXml<'a> {
    pub c: &'static Config,
    pub schema: &'a str,
    pub email: &'a str,
}

#[derive(Template)]
#[template(path = "autodiscover-json.html")]
pub struct AutoDiscoverJson {
    pub c: &'static Config,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AutoDiscoverRequest {
    pub EMailAddress: Option<String>,
    pub AcceptableResponseSchema: Option<String>,
}
