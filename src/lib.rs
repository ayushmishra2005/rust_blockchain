mod block;
pub use block::Block;
pub mod transaction;
pub use transaction::Transaction;
pub mod blockchain;
pub use blockchain::Blockchain;
use std::time::Instant;

const DIFFICULT_LEVEL: i32 = 2;
const MINING_REWARD: f32 = 100f32;

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
/// let time:u64 = 1573978703u64;
/// let pre_hash = "fd1afb6022cd4d47c890961c533928eacfe8219f1b2524f7fb2a61847ddf8c27".to_owned();
/// let transactions = vec![
///        Transaction {
///             sender: String::from("Ryan"),
///             receiver: String::from("Dan"),
///             amount: 2000.0,
///         }
///     ];
/// let nonce :u64= 0u64;
///
/// let hash = calculate_hash(&pre_hash, &transactions, &time, &nonce);
///
/// assert_eq!(hash, "f424364cf2c0df65381337bee2487cfec30b6074a5a0d8b2bcf6695df856e680");
///
/// ```
///
pub fn calculate_hash(
    pre_hash: &str,
    transactions: &Vec<Transaction>,
    timestamp: &u64,
    nonce: &u64,
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
    bytes.extend(&nonce.to_ne_bytes());

    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
}

///
/// Create difficulty string target, to compare with hash
/// # Example
///
/// ```
/// use rustblockchainlib::get_difficult_string;
///
/// let target = get_difficult_string();
/// assert_eq!(target, "00")
/// ```
///

pub fn get_difficult_string() -> String {
    let mut s = String::new();
    for _i in 0..DIFFICULT_LEVEL {
        s.push_str("0");
    }
    s
}
