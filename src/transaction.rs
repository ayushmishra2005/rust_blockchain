use ed25519_dalek::Keypair;
use ed25519_dalek::PublicKey;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::Verifier;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: Option<PublicKey>,
    pub receiver: Option<PublicKey>,
    pub amount: f32,
    pub signature: Option<Signature>,
}

impl Transaction {
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        if let Some(sender) = self.sender {
            bytes.extend(sender.as_bytes());
        }
        if let Some(receiver) = self.receiver {
            bytes.extend(receiver.as_bytes());
        }
        bytes.extend(&self.amount.to_bits().to_ne_bytes());

        bytes
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }

    pub fn sign_transaction(&mut self, key: Keypair) {
        if let Some(p) = self.sender {
            if p != key.public {
                panic!("You can not sign other's transaction!!!")
            } else {
                self.signature = Some(key.sign(&self.calculate_hash()));
            }
        }
    }

    pub fn is_valid_transaction(&self) -> bool {
        match (self.sender, self.signature) {
            (Some(p), Some(s)) if p.verify(&self.calculate_hash(), &s).is_ok() => true,
            (None, _) => true, // For miner reward
            _ => false,
        }
    }
}
