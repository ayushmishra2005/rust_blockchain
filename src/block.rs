use super::*;

#[derive(Debug)]
pub struct Block {
    pub timestamp: u64,
    pub hash: String,
    pub pre_hash: String,
    pub transaction: Vec<Transaction>,
}

impl Block {
    ///
    /// Block constructor
    /// Creates block from previous block hash and transaction data
    ///

    pub fn new(transaction: Vec<Transaction>) -> Self {
        let time = now();
        let pre_hash = String::new();
        Block {
            timestamp: time,
            hash: calculate_hash(&pre_hash, &transaction, &time),
            pre_hash,
            transaction,
        }
    }

    ///
    /// Set hash of previous block as pre hash
    ///
    pub fn set_pre_hash(&mut self, pre_hash: String) {
        self.pre_hash = pre_hash;
    }
}
