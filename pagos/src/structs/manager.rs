use std::sync::{Arc, Mutex};
use crate::structs::client::Cliente;

#[derive(Debug, Clone)]
pub struct Manager {
    pub users: Arc<Mutex<Vec<(Cliente,f64)>>>,
    pub generated: Arc<Mutex<usize>>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            users: Arc::new(Mutex::new(Vec::new())),
            generated: Arc::new(Mutex::new(0))
        }
    }
}

/* 

    pub fn add_client(&mut self, client: Cliente) -> usize {
        let mut users = self.users.lock().unwrap();
        let id = users.len();
        users.push((client, 0));
        id
    }

    pub fn get_client(&self, id: usize) -> Option<Cliente> {
        let users = self.users.lock().unwrap();
        if id < users.len() {
            Some(users[id].0.clone())
        } else {
            None
        }
    }

    pub fn get_client_balance(&self, id: usize) -> Option<usize> {
        let users = self.users.lock().unwrap();
        if id < users.len() {
            Some(users[id].1)
        } else {
            None
        }
    }

    pub fn set_client_balance(&mut self, id: usize, balance: usize) -> bool {
        let mut users = self.users.lock().unwrap();
        if id < users.len() {
            users[id].1 = balance;
            true
        } else {
            false
        }
    }

*/