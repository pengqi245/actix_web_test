use serde::{Deserialize, Serialize};
use chrono::prelude::*;
// use rbatis::crud::{CRUDEnable};

#[derive(Serialize, Deserialize)]
pub struct MyObj {
    pub name: String,
    pub number: String
}

// #[derive(CRUDEnable,Serialize, Deserialize, Clone, Debug)] 
#[derive(Serialize, Deserialize, Clone, Debug)] 
pub struct User {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub number: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)] 
pub struct UserModel {
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub cteate_date: String
}