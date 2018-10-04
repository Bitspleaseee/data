//! The requests a admin can send to the service

use crate::payloads::TokenPayload;
use crate::valid::token::Token;
use std::net::IpAddr;

pub type TokenAdminRequest = TokenPayload<AdminRequest, Token>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AdminRequest {
    BanIp(IpAddrPayload),
    UnbanIp(IpAddrPayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpAddrPayload {
    pub ip: IpAddr,
}
