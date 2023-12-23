use crate::structs::client::Cliente;
use std::sync::{Arc, Mutex};

/// el Struct Manager es el encargado de almacenar los datos de los clientes y sus saldos, tambien lleva a cuenta de cuantos reportes se han generado.
#[derive(Debug, Clone)]
pub struct Manager {
    pub users: Arc<Mutex<Vec<(Cliente, f64)>>>,
    pub generated: Arc<Mutex<usize>>,
}

impl Manager {
    /// Crea una nueva instancia de Manager
    pub fn new() -> Manager {
        Manager {
            users: Arc::new(Mutex::new(Vec::new())),
            generated: Arc::new(Mutex::new(0)),
        }
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}
