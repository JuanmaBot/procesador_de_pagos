use serde::{Deserialize, Serialize};

/// Cliente struct es usado para almacenar los datos de los clientes.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cliente {
    pub client_name: String,
    pub birth_date: String,
    pub document_number: String,
    pub country: String,
}
