use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyObj {
    pub name: String,
    pub number: String
}