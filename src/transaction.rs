#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}

impl Transaction {
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.sender.as_bytes());
        bytes.extend(self.receiver.as_bytes());
        bytes.extend(&self.amount.to_bits().to_ne_bytes());

        bytes
    }
}
