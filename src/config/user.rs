use std::io;
use std::fs;
use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub username: String, 
    pub email: String
}
