use std::fs::File;
use std::io::Read;
use std::fmt;
use serde::{Deserialize, Deserializer};
use crate::serde::de::Error;

lazy_static! {
    pub static ref CONFIG: Config = Config::init();
}

#[derive(Deserialize, Debug)]
pub enum EncryptionType {
    None, SSL, StartTLS
}

impl fmt::Display for EncryptionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug)]
pub enum LoginType {
    Localpart, EmailDom
}

impl fmt::Display for LoginType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn deser_logintype<'a, D>(de: D) -> Result<LoginType, D::Error>
    where D: Deserializer<'a>
{
    let s: String = Deserialize::deserialize(de)?;
    match s.to_lowercase().as_ref() {
        "localpart" => Ok(LoginType::Localpart),
        "emaildom" => Ok(LoginType::EmailDom),
        other => Err(D::Error::custom(format!("unknown login type: {}", other))),
    }
}

fn deser_encryptiontype<'a, D>(de: D) -> Result<EncryptionType, D::Error>
    where D: Deserializer<'a>
{
    let s: String = Deserialize::deserialize(de)?;
    match s.to_lowercase().as_ref() {
        "none" => Ok(EncryptionType::None),
        "ssl" => Ok(EncryptionType::SSL),
        "starttls" => Ok(EncryptionType::StartTLS),
        other => Err(D::Error::custom(format!("unknown encryption type: {}", other))),
    }
}

#[derive(Deserialize)]
pub struct ConfigGeneral {
    pub listening_address: String,
    pub domain: String,
    pub full_name: String,
    pub short_name: String,
    pub support_url: String,
    pub debug_mode: bool,
}

#[derive(Deserialize)]
pub struct ConfigAutodiscover {
    pub enabled: bool,
    pub address: String,
    pub ttl: u16,
}

#[derive(Deserialize)]
pub struct ConfigAutoconfig {
    pub enabled: bool,
    pub support_descr: String,
}

#[derive(Deserialize)]
pub struct ConfigImap {
    pub enabled: bool,
    pub hostname: String,
    pub port: u16,
    #[serde(deserialize_with = "deser_logintype")]
    pub login: LoginType,
    #[serde(deserialize_with = "deser_encryptiontype")]
    pub encryption: EncryptionType,
    pub microsoft_password_encryption: bool,
    pub standard_password_encryption: bool,
}

#[derive(Deserialize)]
pub struct ConfigImaps {
    pub enabled: bool,
    pub hostname: String,
    pub port: u16,
    #[serde(deserialize_with = "deser_logintype")]
    pub login: LoginType,
    #[serde(deserialize_with = "deser_encryptiontype")]
    pub encryption: EncryptionType,
    pub microsoft_password_encryption: bool,
    pub standard_password_encryption: bool,
}

#[derive(Deserialize)]
pub struct ConfigPop {
    pub enabled: bool,
    pub hostname: String,
    pub port: u16,
    #[serde(deserialize_with = "deser_logintype")]
    pub login: LoginType,
    #[serde(deserialize_with = "deser_encryptiontype")]
    pub encryption: EncryptionType,
    pub microsoft_password_encryption: bool,
    pub standard_password_encryption: bool,
}

#[derive(Deserialize)]
pub struct ConfigPops {
    pub enabled: bool,
    pub hostname: String,
    pub port: u16,
    #[serde(deserialize_with = "deser_logintype")]
    pub login: LoginType,
    #[serde(deserialize_with = "deser_encryptiontype")]
    pub encryption: EncryptionType,
    pub microsoft_password_encryption: bool,
    pub standard_password_encryption: bool,
}

#[derive(Deserialize)]
pub struct ConfigSmtp {
    pub enabled: bool,
    pub hostname: String,
    pub port: u16,
    #[serde(deserialize_with = "deser_logintype")]
    pub login: LoginType,
    #[serde(deserialize_with = "deser_encryptiontype")]
    pub encryption: EncryptionType,
    pub microsoft_password_encryption: bool,
    pub standard_password_encryption: bool,
}

#[derive(Deserialize)]
pub struct ConfigSmtps {
    pub enabled: bool,
    pub hostname: String,
    pub port: u16,
    #[serde(deserialize_with = "deser_logintype")]
    pub login: LoginType,
    #[serde(deserialize_with = "deser_encryptiontype")]
    pub encryption: EncryptionType,
    pub microsoft_password_encryption: bool,
    pub standard_password_encryption: bool,
}

#[derive(Deserialize)]
pub struct ConfigSubmission {
    pub enabled: bool,
    pub hostname: String,
    pub port: u16,
    #[serde(deserialize_with = "deser_logintype")]
    pub login: LoginType,
    #[serde(deserialize_with = "deser_encryptiontype")]
    pub encryption: EncryptionType,
    pub microsoft_password_encryption: bool,
    pub standard_password_encryption: bool,
}

#[derive(Deserialize)]
pub struct Config {
    pub general: ConfigGeneral,
    pub autodiscover: ConfigAutodiscover,
    pub autoconfig: ConfigAutoconfig,
    pub imap: ConfigImap,
    pub imaps: ConfigImaps,
    pub pop: ConfigPop,
    pub pops: ConfigPops,
    pub smtp: ConfigSmtp,
    pub smtps: ConfigSmtps,
    pub submission: ConfigSubmission,
}

impl Config {
    pub fn init() -> Self {
        let mut conffile = File::open("config.toml").expect(
            "Config file config.toml not found."
        );
        let mut confstr = String::new();
        conffile
            .read_to_string(&mut confstr)
            .expect("Couldn't read config to string");
        toml::from_str(&confstr).expect("The configuration file seems invalid. Please double check it!")
    }
}
