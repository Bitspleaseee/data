//! The requests a user can send to the auth-service

use crate::payloads::EmptyPayload;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AuthRequest<'a> {
    Authenticate(#[serde(borrow)] AuthReqPayload<'a>),
    Deauthenticate(EmptyPayload),
    RegisterUser(#[serde(borrow)] RegisterPayload<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "payload")]
pub struct AuthReqPayload<'a> {
    #[serde(rename = "username")]
    pub raw_username: &'a str,
    #[serde(rename = "password")]
    pub raw_password: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "payload")]
pub struct RegisterPayload<'a> {
    #[serde(rename = "username")]
    pub raw_username: &'a str,
    #[serde(rename = "password")]
    pub raw_password: &'a str,
    #[serde(rename = "email")]
    pub raw_email: &'a str,
}