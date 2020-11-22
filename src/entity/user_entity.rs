use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)] 
pub struct UserEntity {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub cteate_date: Option<NaiveDateTime>
}