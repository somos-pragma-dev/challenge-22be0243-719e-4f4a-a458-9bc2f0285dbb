use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub account_number: String,
    pub holder_name: String,
    pub balance: f64,
    pub account_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewAccount {
    pub account_number: String,
    pub holder_name: String,
    pub balance: f64,
    pub account_type: String,
}