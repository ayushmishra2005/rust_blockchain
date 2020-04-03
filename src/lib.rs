
mod block;
pub use block::Block;
pub mod transaction;
pub use transaction::Transaction;
pub mod blockchain;
pub use blockchain::Blockchain;
use std::time::Instant;

pub fn now() -> u64 {
    Instant::now().elapsed().as_secs()
}

/// Calculate crypto hash of block
///  # Example
///
/// ```
/// use rustblockchainlib::now;
/// use rustblockchainlib::calculate_hash;
/// use rustblockchainlib::Transaction;
///
/// let time = 1573978703;
/// let pre_hash = "fd1afb6022cd4d47c890961c533928eacfe8219f1b2524f7fb2a61847ddf8c27".to_owned();
/// let transactions = vec![
///        Transaction {
///             sender: String::from("Ryan"),
///             receiver: String::from("Dan"),
///             amount: 2000.0,
///         }
///     ];
///
/// let hash = calculate_hash(&pre_hash, &transactions, time);
///
/// assert_eq!(hash, "5ac248220eb5f332f5ea73d1c5ba4f9f24aac8c3f8bcc4cc2ddd95012cc5acce");
///
/// ```
///
pub fn calculate_hash(
    pre_hash: &String,
    transactions: &Vec<Transaction>,
    timestamp: u64,
) -> String {
    let mut bytes = vec![];
    bytes.extend(&timestamp.to_ne_bytes());
    bytes.extend(
        transactions
            .iter()
            .flat_map(|transaction| transaction.bytes())
            .collect::<Vec<u8>>(),
    );
    bytes.extend(pre_hash.as_bytes());

    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
}
