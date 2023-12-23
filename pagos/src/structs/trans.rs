struct Transaction {
    client_id: usize,
    credit_amount: f64,
}

impl Transaction {
    fn new(client_id: usize, credit_amount: f64) -> Self {
        Self {
            client_id,
            credit_amount,
        }
    }
}