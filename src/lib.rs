use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, Promise, log, ext_contract, Gas, PromiseResult
};
use near_sdk::PromiseResult::Successful;
use near_sdk::serde_json::json;
use serde_json;
use near_sdk::env::log_str;
use std::convert::TryInto;
use bs58;
use near_sdk::json_types::{Base64VecU8, U128};
use sbv2_near::{AggregatorRound, Uuid, SWITCHBOARD_PROGRAM_ID};
pub type AssetId = String;

// Convertir valor en formato json
macro_rules! json_buf {
    ($x:tt) => {
        json!($x).to_string().as_bytes().to_vec()
    };
}

// Estructura Ix
#[derive(Deserialize, Serialize)]
pub struct Ix {
    pub address: [u8; 32],
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {}

#[near_bindgen]
impl Contract {

    // Metodo para la consulta de una fuente de datos (recibe un valor de tipo Ix)
    #[payable]
    pub fn aggregator_read(&mut self, ix: Ix) -> Promise {
        // Hacemos uso de las llamadas de contratos cruzados hacia el contrato de switchboard
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
                // Al finalizar la primer llamada mandamos a llamar el metodo callback para recuperar el valor.
                Promise::new(near_sdk::env::current_account_id()).function_call(
                    "callback".into(),
                    json_buf!({}),
                    near_sdk::ONE_YOCTO,
                    near_sdk::Gas(30000000000000),
                ),
            )
    }

    // Metodo para la consulta de una fuente de datos (recibe un address)
    #[payable]
    pub fn feed_read(&mut self, feed_address: String) -> Promise {
        // Mandamos a llamar el metodo para convertir el address a un vector de bytes
        let address = Contract::near_address_to_bytes32(feed_address.clone());

        // Hacemos uso de las llamadas de contratos cruzados hacia el contrato de switchboard
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
                // Al finalizar la primer llamada mandamos a llamar el metodo callback para recuperar el valor.
                Promise::new(near_sdk::env::current_account_id()).function_call(
                    "callback".into(),
                    json_buf!({}),
                    near_sdk::ONE_YOCTO,
                    near_sdk::Gas(30000000000000),
                ),
            )
    }

    #[payable]
    // Es la función que se llamará después de que el contrato externo switchboard-v2.testnet haya terminado de ejecutarse
    pub fn callback(&mut self) {
        // Esta función obtiene el resultado de la promesa que se registró anteriormente
        let response = near_sdk::env::promise_result(0);
        if let Successful(serialized_round) = response {
            //Convierte el resultado en un valor de tipo AggregatorRound que viene de sbv2_near
            let round: AggregatorRound = serde_json::from_slice(&serialized_round).unwrap();
            // Convierte el valor numérico de ese objeto en un tipo f64
            let val: f64 = round.result.try_into().unwrap();
            log!("Valor del Oráculo: {:?}", val);
        } else {
            log_str("Error al obtener valor del Oráculo");
        }
    }

    // Convertir address a una matriz de bytes de 32 bytes de largo
    pub fn near_address_to_bytes32(address: String) -> [u8; 32] {
        // Decodifica el address en formato Base58 en un vector de bytes utilizando la biblioteca bs58
        let decoded = bs58::decode(address).into_vec().unwrap();
    
        // Convierte el vector de bytes en una matriz de bytes de 32 bytes de largo
        let bytes32: [u8; 32] = decoded
        .try_into()
        .expect("Address should be exactly 32 bytes long");
    
        // Retorna el vector
        return bytes32;
    }
    
    // Consultar Oraculo de Promixityfi
    pub fn get_asset_promixityfi(&self, asset_id: String) -> Promise {
        // Hacemos uso de las llamadas de contratos cruzados hacia el contrato de proximityfi
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