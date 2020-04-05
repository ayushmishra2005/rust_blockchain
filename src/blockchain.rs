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

        self.blocks.push(block)
    }
}
