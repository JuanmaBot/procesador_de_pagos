use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Transaction {
    pub client_id: usize,
    pub credit_amount: f64,
}