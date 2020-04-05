use super::*;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, mut block: Block) {
        match self.blocks.last() {
            Some(pre_block) => block.set_pre_hash(pre_block.hash.to_owned()),
            None => block.set_pre_hash("0".to_string()), // genesis_block
        }
        block.set_hash();
        block.mine();
        self.blocks.push(block)
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
