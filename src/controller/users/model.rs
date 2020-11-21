use serde::{Deserialize, Serialize};
use chrono::prelude::*;
// use rbatis::crud::{CRUDEnable};

#[derive(Serialize, Deserialize)]
pub struct MyObj {
    pub name: String,
    pub number: String
}

#[derive(Serialize, Deserialize, Clone, Debug)] 
pub struct UserModel {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub cteate_date: Option<NaiveDateTime>
}