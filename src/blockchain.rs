use super::*;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    unmined_transactions: Vec<Transaction>,
    mining_reward: f32,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unmined_transactions: vec![],
            mining_reward: MINING_REWARD,
        }
    }

    pub fn mine_unmined_transactions(&mut self, miner_address: String) {
        let transactions = &self.unmined_transactions;
        let mut block = Block::new(transactions.to_vec());
        match self.blocks.last() {
            Some(pre_block) => block.set_pre_hash(pre_block.hash.to_owned()),
            None => block.set_pre_hash("0".to_string()), // genesis_block
        }
        block.set_hash();
        block.mine();
        self.blocks.push(block);
        self.unmined_transactions = vec![Transaction {
            sender: String::new(),
            receiver: miner_address,
            amount: self.mining_reward,
        }];
    }

    pub fn add_new_transaction(&mut self, transaction: Transaction) {
        self.unmined_transactions.push(transaction);
    }

    pub fn is_valid_chain(&self) -> bool {
        let blocks = &self.blocks;

        for (i, block) in blocks.iter().enumerate() {
            if block.hash
                != calculate_hash(
                    &block.pre_hash,
                    &block.transaction,
                    &block.timestamp,
                    &block.nonce,
                )
            {
                return false;
            }
            if i > 0 && block.pre_hash != blocks[i - 1].hash {
                return false;
            }
        }

        return true;
    }
}
