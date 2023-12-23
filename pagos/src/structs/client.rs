use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cliente {
    pub client_name: String,
    pub birth_date: String,
    pub document_number: String,
    pub country: String,
}