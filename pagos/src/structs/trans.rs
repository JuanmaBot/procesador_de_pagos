use serde::{Deserialize, Serialize};

/// Transaction struct es usado para almacenar los datos de las transacciones de los clientes.
#[derive(Serialize, Debug, Deserialize)]
pub struct Transaction {
    pub client_id: usize,
    pub credit_amount: f64,
}
