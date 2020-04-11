use rustblockchainlib::*;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_new_transaction(Transaction {
        sender: String::from("Ryan"),
        receiver: String::from("Dan"),
        amount: 2000.0,
    });

    blockchain.mine_unmined_transactions("First Miner".to_owned());

    println!("{}", blockchain.is_valid_chain());
    println!("{:#?}", blockchain);

    blockchain.add_new_transaction(Transaction {
        sender: String::from("Sam"),
        receiver: String::from("Michal"),
        amount: 2500.0,
    });

    blockchain.mine_unmined_transactions("Second Miner".to_owned());

    println!("{}", blockchain.is_valid_chain());
    println!("{:#?}", blockchain);

    blockchain.add_new_transaction(Transaction {
        sender: String::from("Michal"),
        receiver: String::from("Dan"),
        amount: 1000.0,
    });

    blockchain.mine_unmined_transactions("Third Miner".to_owned());

    println!("{}", blockchain.is_valid_chain());
    println!("{:#?}", blockchain);
}
