use crate::counter_caller::CounterCaller;
use alloy_primitives::U256;
use serde::{Serialize, Deserialize};
use kinode_process_lib::{Address, get_state, set_state, eth::{Log, }};
use std::collections::HashMap;
use alloy_signer::{k256::ecdsa::SigningKey, LocalWallet, Signer};
use crate::counter_caller::NumberIncrementedLog;

// from UI to backend
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsPush {
    SetNumber(U256),
    Increment,
    Number,
}

// from backend to UI
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsUpdate {
    Number(U256),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    EncryptWallet {private_key: Option<String>, password: String}, // if none, will use decrypted wallet key
    DecryptWallet(String),
    GetLogs(u64), // from block
    ManyIncrements(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivateKey {
    Encrypted(Vec<u8>),
    Decrypted(Wallet),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub address: String,
    pub private_key: String,
}

impl From<LocalWallet> for Wallet {
    fn from(wallet: LocalWallet) -> Self {
        Wallet {
            address: wallet.address().to_string(),
            private_key: hex::encode(wallet.to_bytes()),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub our: Address,
    pub wallets: HashMap<u64, PrivateKey>, //chain id to wallet
    pub increment_log_index: Vec<NumberIncrementedLog>
}

impl State {
    pub fn new(our: &Address) -> Self {
        State {
            our: our.clone(),
            wallets: HashMap::new(),
            increment_log_index: Vec::new(),
        }
    }
    pub fn fetch() -> Option<State> {
        if let Some(state_bytes) = get_state() {
            serde_json::from_slice(&state_bytes).ok()
        } else {
            None
        }
    }
    pub fn save(&self) {
        let serialized_state = serde_json::to_string(self).expect("Failed to serialize state");
        set_state(&serialized_state.into_bytes());
    }
}
