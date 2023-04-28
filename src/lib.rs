use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log_str;
use near_sdk::log;
use near_sdk::near_bindgen;
use near_sdk::serde_json::json;
use near_sdk::Promise;
use near_sdk::PromiseResult::Successful;
use serde::{Deserialize, Serialize};
use serde_json;
use std::convert::TryInto;
use sbv2_near::{AggregatorRound, Uuid, SWITCHBOARD_PROGRAM_ID};
use bs58;
use near_sdk::Balance;
use near_sdk::AccountId;
use near_sdk::{ext_contract, Gas, PromiseResult};
use near_sdk::json_types::{Base64VecU8, U128};
pub type AssetId = String;
use near_sdk::env;
use core::any::Any;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

macro_rules! json_buf {
    ($x:tt) => {
        json!($x).to_string().as_bytes().to_vec()
    };
}

#[derive(Deserialize, Serialize)]
pub struct Ix {
    pub address: [u8; 32],
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn aggregator_read(&mut self, ix: Ix) -> Promise {
        Promise::new("switchboard-v2.testnet".parse().unwrap())
            .function_call(
                "aggregator_read".into(),
                json_buf!({
                    "ix": {
                        "address": ix.address,
                        "payer": ix.address,
                    }
                }),
                near_sdk::ONE_YOCTO,
                near_sdk::Gas(30000000000000),
            )
            .then(
                Promise::new(near_sdk::env::current_account_id()).function_call(
                    "callback".into(),
                    json_buf!({}),
                    near_sdk::ONE_YOCTO,
                    near_sdk::Gas(30000000000000),
                ),
            )
    }

    #[payable]
    pub fn feed_read(&mut self, feed_address: String) -> Promise {
        let address = Contract::near_address_to_bytes32(feed_address.clone());

        Promise::new("switchboard-v2.testnet".parse().unwrap())
            .function_call(
                "aggregator_read".into(),
                json_buf!({
                    "ix": {
                        "address": address,
                        "payer": address,
                    }
                }),
                near_sdk::ONE_YOCTO,
                near_sdk::Gas(30000000000000),
            )
            .then(
                Promise::new(near_sdk::env::current_account_id()).function_call(
                    "callback".into(),
                    json_buf!({}),
                    near_sdk::ONE_YOCTO,
                    near_sdk::Gas(30000000000000),
                ),
            )
    }

    #[payable]
    pub fn callback(&mut self) {
        let maybe_round = near_sdk::env::promise_result(0);
        if let Successful(serialized_round) = maybe_round {
            let round: AggregatorRound = serde_json::from_slice(&serialized_round).unwrap();
            let val: f64 = round.result.try_into().unwrap();
            log!("Feed value: {:?}", val);
        } else {
            log_str("error");
        }
    }

    // Convertir feed address to [u8; 32]
    pub fn near_address_to_bytes32(address: String) -> [u8; 32] {
        // Decode the base58-encoded address string
        let decoded = bs58::decode(address).into_vec().unwrap();
    
        // Serialize the decoded address into a byte array
        let bytes32: [u8; 32] = decoded
        .try_into()
        .expect("Address should be exactly 32 bytes long");
    
        bytes32
    }
    
    // Consultar Oraculo de Promixityfi
    pub fn get_asset_promixityfi(&self, asset_id: String) -> Promise {
        Promise::new("priceoracle.testnet".parse().unwrap())
        .function_call(
            "get_asset".into(),
            json_buf!({
                "asset_id": asset_id
            }),
            0,
            near_sdk::Gas(30000000000000),
        )
    }
}