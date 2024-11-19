mod utils;

use std::{cell::RefCell, collections::BTreeSet, io::Write, rc::Rc};

use bdk_esplora::{
    esplora_client::{self, AsyncClient},
    EsploraAsyncExt,
};
use bdk_wallet::{bitcoin::Network, ChangeSet, KeychainKind, Wallet};
use js_sys::Date;
use wasm_bindgen::prelude::*;
use web_sys::console;

const PARALLEL_REQUESTS: usize = 1;

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, bdk-wasm!".into()
}

#[wasm_bindgen]
pub struct WalletWrapper {
    wallet: Rc<RefCell<Wallet>>,
    client: Rc<RefCell<AsyncClient>>,
}

#[wasm_bindgen]
impl WalletWrapper {
    // --8<-- [start:new]
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
            wallet: Rc::new(RefCell::new(wallet)),
            client: Rc::new(RefCell::new(client)),
        })
    }
    // --8<-- [end:new]

    // --8<-- [start:scan]
    #[wasm_bindgen]
    pub async fn sync(&self, stop_gap: usize) -> Result<(), String> {
        let wallet = Rc::clone(&self.wallet);
        let client = Rc::clone(&self.client);

        let request = wallet.borrow().start_full_scan().inspect({
            let mut stdout = std::io::stdout();
            let mut once = BTreeSet::<KeychainKind>::new();
            move |keychain, spk_i, _| {
                if once.insert(keychain) {
                    console::log_1(&format!("\nScanning keychain [{:?}]", keychain).into());
                }
                console::log_1(&format!(" {:<3}", spk_i).into());
                stdout.flush().expect("must flush")
            }
        });

        let update = client
            .borrow()
            .full_scan(request, stop_gap, PARALLEL_REQUESTS)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let now = (Date::now() / 1000.0) as u64;
        wallet
            .borrow_mut()
            .apply_update_at(update, Some(now))
            .map_err(|e| format!("{:?}", e))?;

        console::log_1(&"after apply".into());

        Ok(())
    }
    // --8<-- [end:scan]

    // --8<-- [start:utils]
    #[wasm_bindgen]
    pub fn balance(&self) -> u64 {
        let balance = self.wallet.borrow().balance();
        balance.total().to_sat()
    }

    #[wasm_bindgen]
    pub fn get_new_address(&self) -> String {
        let address = self
            .wallet
            .borrow_mut()
            .next_unused_address(KeychainKind::External);

        address.to_string()
    }
    // --8<-- [end:utils]

    #[wasm_bindgen]
    pub fn peek_address(&self, index: u32) -> String {
        let address = self
            .wallet
            .borrow_mut()
            .peek_address(KeychainKind::External, index);

        address.to_string()
    }
}
