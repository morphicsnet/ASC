use asc_types::model::KernelOutput;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    pub seq: u64,
    pub payload: KernelOutput,
    pub prev_hash: String,
    pub hash: String,
}

#[derive(Debug, Default)]
pub struct EventLog {
    pub records: Vec<EventRecord>,
    pub tip_hash: String,
}

impl EventLog {
    pub fn append(&mut self, seq: u64, payload: &KernelOutput) {
        let prev_hash = self.tip_hash.clone();
        let bytes =
            serde_json::to_vec(&(seq, payload, &prev_hash)).expect("serialize event record");
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let hash = hex::encode(hasher.finalize());
        self.records.push(EventRecord {
            seq,
            payload: payload.clone(),
            prev_hash,
            hash: hash.clone(),
        });
        self.tip_hash = hash;
    }
}
