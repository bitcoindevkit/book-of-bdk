use std::iter::FromIterator;
use bdk_esplora::{
    esplora_client::{self, AsyncClient},
    EsploraAsyncExt,
};
use bdk_wallet::{chain::Merge, bitcoin::Network, ChangeSet, KeychainKind, Wallet};
use js_sys::Date;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};
use serde_json::{self, Value};

const PARALLEL_REQUESTS: usize = 1;

pub type JsResult<T> = Result<T, JsError>;

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, bdk-wasm!".into()
}

#[wasm_bindgen]
pub struct WalletWrapper {
    wallet: Wallet,
    client: AsyncClient,
}

#[wasm_bindgen]
impl WalletWrapper {
    // --8<-- [start:wallet]
    #[wasm_bindgen(constructor)]
    pub fn new(
        network: String,
        external_descriptor: String,
        internal_descriptor: String,
        esplora_url: String,
    ) -> Result<WalletWrapper, String> {
        let network = match network.as_str() {
            "mainnet" => Network::Bitcoin,
            "testnet" => Network::Testnet,
            "testnet4" => Network::Testnet4,
            "signet" => Network::Signet,
            "regtest" => Network::Regtest,
            _ => return Err("Invalid network".into()),
        };

        let wallet_opt = Wallet::load()
            .descriptor(KeychainKind::External, Some(external_descriptor.clone()))
            .descriptor(KeychainKind::Internal, Some(internal_descriptor.clone()))
            .extract_keys()
            .check_network(network)
            .load_wallet_no_persist(ChangeSet::default())
            .map_err(|e| format!("{:?}", e))?;

        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => Wallet::create(external_descriptor, internal_descriptor)
                .network(network)
                .create_wallet_no_persist()
                .map_err(|e| format!("{:?}", e))?,
        };

        let client = esplora_client::Builder::new(&esplora_url)
            .max_retries(6)
            .build_async()
            .map_err(|e| format!("{:?}", e))?;

        Ok(WalletWrapper {
            wallet: wallet,
            client: client,
        })
    }

    pub fn load(changeset_str: &str, url: &str, external_descriptor: &str, internal_descriptor: &str) -> JsResult<WalletWrapper> {
        // Parse the JSON string and restore maps
        let changeset_value: Value = serde_json::from_str(changeset_str)?;
        let restored_value = restore_maps(changeset_value);
        let changeset: ChangeSet = serde_json::from_value(restored_value)?;

        let wallet_opt = Wallet::load()
            .descriptor(KeychainKind::External, Some(external_descriptor.to_string()))
            .descriptor(KeychainKind::Internal, Some(internal_descriptor.to_string()))
            .extract_keys()
            .load_wallet_no_persist(changeset)?;

        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => return Err(JsError::new("Failed to load wallet, check the changeset")),
        };

        let client = esplora_client::Builder::new(&url).build_async()?;

        Ok(WalletWrapper { wallet, client })
    }

    pub async fn scan(&mut self, stop_gap: usize) -> Result<(), String> {
        let wallet = &mut self.wallet;
        let client = &self.client;

        let request = wallet.start_full_scan();

        let update = client
            .full_scan(request, stop_gap, PARALLEL_REQUESTS)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let now = (Date::now() / 1000.0) as u64;
        wallet
            .apply_update_at(update, Some(now))
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    pub async fn sync(&mut self, parallel_requests: usize) -> JsResult<()> {
        let request = self.wallet.start_sync_with_revealed_spks();
        let update = self.client.sync(request, parallel_requests).await?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet.apply_update_at(update, Some(now))?;

        Ok(())
    }
    // --8<-- [end:wallet]

    // --8<-- [start:utils]
    pub fn balance(&self) -> u64 {
        let balance = self.wallet.balance();
        balance.total().to_sat()
    }

    pub fn get_new_address(&mut self) -> String {
        let address = self
            .wallet
            .reveal_next_address(KeychainKind::External);

        address.to_string()
    }
    // --8<-- [end:utils]

    pub fn peek_address(&mut self, index: u32) -> String {
        let address = self
            .wallet
            .peek_address(KeychainKind::External, index);

        address.to_string()
    }

    // --8<-- [start:store]
    pub fn take_staged(&mut self) -> JsResult<String> {
        match self.wallet.take_staged() {
            Some(changeset) => {
                // First convert to a generic Value that we can modify
                let mut value = serde_json::to_value(&changeset)?;
                
                // Handle Map serialization
                transform_maps(&mut value);
                
                // Convert to JSON string
                Ok(serde_json::to_string(&value)?)
            }
            None => Ok("null".to_string()),
        }
    }

    pub fn take_merged(&mut self, previous: String) -> JsResult<String> {
        match self.wallet.take_staged() {
            Some(curr_changeset) => {
                // Parse the previous JSON string
                let previous_value: Value = serde_json::from_str(&previous)?;
                
                // Convert back from our custom Map format
                let mut previous_changeset: ChangeSet = 
                    serde_json::from_value(restore_maps(previous_value))?;
                
                previous_changeset.merge(curr_changeset);
                
                // Convert to Value and handle Map serialization
                let mut final_value = serde_json::to_value(&previous_changeset)?;
                transform_maps(&mut final_value);
                
                // Convert to JSON string
                Ok(serde_json::to_string(&final_value)?)
            }
            None => Ok("null".to_string()),
        }
    }
    // --8<-- [end:store]
}

fn transform_maps(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                transform_maps(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                transform_maps(v);
            }
        }
        Value::Object(obj) if obj.contains_key("entries") => {
            // This assumes Map-like structures have an "entries" field
            *value = Value::Object(serde_json::Map::from_iter([
                ("dataType".to_string(), Value::String("Map".to_string())),
                ("value".to_string(), obj["entries"].clone()),
            ]));
        }
        _ => {}
    }
}

fn restore_maps(mut value: Value) -> Value {
    match &value {
        Value::Object(map) => {
            if map.get("dataType").and_then(Value::as_str) == Some("Map") {
                if let Some(entries) = map.get("value") {
                    let mut new_obj = serde_json::Map::new();
                    new_obj.insert("entries".to_string(), entries.clone());
                    Value::Object(new_obj)
                } else {
                    value
                }
            } else {
                let mut new_map = serde_json::Map::new();
                for (k, v) in map {
                    new_map.insert(k.clone(), restore_maps(v.clone()));
                }
                Value::Object(new_map)
            }
        }
        Value::Array(arr) => {
            Value::Array(arr.iter().map(|v| restore_maps(v.clone())).collect())
        }
        _ => value,
    }
}