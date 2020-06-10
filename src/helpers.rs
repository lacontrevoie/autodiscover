use crate::structs::*;
use std::collections::HashMap;

pub fn get_schema(postdata: Option<AutoDiscoverRequest>) -> String {
    let default_sch = "http://schemas.microsoft.com/exchange/autodiscover/outlook/responseschema/2006a";
    if let Some(v) = postdata {
        match &v.AcceptableResponseSchema {
            Some(v) => v.to_owned(),
            None => default_sch.to_owned(),
        }
    } else {
        default_sch.to_owned()
    }
}

pub fn get_email_address(postdata: Option<AutoDiscoverRequest>,
    getdata: HashMap<String, String>) -> String {
    // first tries to parse POST data, then GET(EMailAddress), then GET(email)
    if let Some(v) = postdata {
        if let Some(w) = v.EMailAddress {
            return w.to_owned();
        }
    }

    if let Some(x) = getdata.get("EMailAddress") {
        x.to_owned()
    } else if let Some(y) = getdata.get("email") {
        y.to_owned()
    } else {
        "email@example.com".to_owned()
    }
}
