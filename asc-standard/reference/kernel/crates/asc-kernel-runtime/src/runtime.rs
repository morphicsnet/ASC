use anyhow::Result;
use asc_contract::load_contract;
use asc_kernel_model::constrain;
use asc_logging::EventLog;
use asc_types::model::{KernelInput, KernelOutput};
use std::path::Path;

pub struct Runtime {
    contract_fingerprint: String,
    last_tick_ts_ms: Option<u64>,
    pub log: EventLog,
}

impl Runtime {
    pub fn new(contract_fingerprint: String) -> Self {
        Self {
            contract_fingerprint,
            last_tick_ts_ms: None,
            log: EventLog::default(),
        }
    }

    pub fn from_repo(repo_root: &Path, profile_name: &str) -> Result<Self> {
        let bundle = load_contract(repo_root, profile_name)?;
        Ok(Self::new(bundle.fingerprint))
    }

    pub fn evaluate(&mut self, input: &KernelInput) -> KernelOutput {
        let inter_tick_ms = self
            .last_tick_ts_ms
            .map(|prev| input.tick.ts_ms.saturating_sub(prev));

        let mut output = constrain(input, inter_tick_ms);
        output.contract_fingerprint = self.contract_fingerprint.clone();
        self.log.append(input.tick.seq, &output);
        self.last_tick_ts_ms = Some(input.tick.ts_ms);
        output
    }

    pub fn tip_hash(&self) -> String {
        self.log.tip_hash.clone()
    }
}
