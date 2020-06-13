use quick_xml::Reader;
use quick_xml::events::Event;
use actix_web::web::Bytes;
use std::collections::HashMap;
use std::io::Cursor;

use crate::structs::*;

pub fn read_xml(data: Bytes) -> AutoDiscoverRequest {
    let mut reader = Reader::from_reader(Cursor::new(data));
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break AutoDiscoverRequest {
                EMailAddress: match txt.get(0) {
                    Some (a) => Some(a.to_owned()),
                    None => None,
                },
                AcceptableResponseSchema: match txt.get(1) {
                    Some (b) => Some(b.to_owned()),
                    None => None,
                },
            }, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        buf.clear();
        }
    }

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
