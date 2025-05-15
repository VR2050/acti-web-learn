use core::str;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize)]
pub struct User {
    pub uname: String,
    pub passwd: String,
}

#[derive(Serialize)]
pub struct LoginStatus {
    pub status: bool,
}

#[derive(Serialize)]
pub struct RegisterStatus {
    pub status: bool,
}
impl RegisterStatus {
    pub fn new(status: bool) -> Self {
        Self { status: status }
    }
}

impl LoginStatus {
    pub fn new(status: bool) -> Self {
        Self { status: status }
    }
}

impl User {
    pub fn new(uanme: &str, passwd: &str) -> Self {
        Self {
            uname: uanme.to_string(),
            passwd: passwd.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Register_User {
    pub email: String,
    pub passwd: String,
}

impl Register_User {
    pub fn new(email: &str, passwd: &str) -> Self {
        Self {
            email: email.to_string(),
            passwd: passwd.to_string(),
        }
    }
}
