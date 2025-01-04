use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
}
