use bdk_esplora::esplora_client::Sleeper;
use bdk_esplora::{
    esplora_client::{self, AsyncClient},
    EsploraAsyncExt,
};
use bdk_wallet::{bitcoin::Network, chain::Merge, ChangeSet, KeychainKind, Wallet};
use gloo_timers::future::{sleep, TimeoutFuture};
use js_sys::Date;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use wasm_bindgen::prelude::*;
// use serde_wasm_bindgen::{from_value, to_value};
use serde_json::{self, Value};

const PARALLEL_REQUESTS: usize = 1;

pub type JsResult<T> = Result<T, JsError>;

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn setTimeout(callback: &Closure<dyn FnMut()>, timeout: u32) -> i32;
}

#[derive(Clone)]
struct WebSleeper;

// Implement Sleeper trait using JS sleep function, from `bdk-wasm`
impl Sleeper for WebSleeper {
    type Sleep = SendSyncWrapper<TimeoutFuture>;

    fn sleep(dur: Duration) -> Self::Sleep {
        SendSyncWrapper(sleep(dur))
    }
}

// Wrap a future that is not `Send` or `Sync` and make it `Send` and `Sync`, from `bdk-wasm`
pub struct SendSyncWrapper<F>(pub F);

unsafe impl<F> Send for SendSyncWrapper<F> {}
unsafe impl<F> Sync for SendSyncWrapper<F> {}

impl<F> Future for SendSyncWrapper<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: Since we're in a single-threaded WASM environment, this is safe.
        unsafe {
            let this = self.get_unchecked_mut();
            Pin::new_unchecked(&mut this.0).poll(cx)
        }
    }
}

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, bdk-wasm!".into()
}

#[wasm_bindgen]
pub struct WalletWrapper {
    wallet: Wallet,
    client: AsyncClient<WebSleeper>,
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
            .build_async_with_sleeper()
            .map_err(|e| format!("{:?}", e))?;

        Ok(WalletWrapper { wallet, client })
    }

    pub fn load(
        changeset_str: &str,
        url: &str,
        external_descriptor: &str,
        internal_descriptor: &str,
    ) -> JsResult<WalletWrapper> {
        let changeset_value: Value = serde_json::from_str(changeset_str)?;
        let changeset: ChangeSet = serde_json::from_value(changeset_value)?;

        let wallet_opt = Wallet::load()
            .descriptor(
                KeychainKind::External,
                Some(external_descriptor.to_string()),
            )
            .descriptor(
                KeychainKind::Internal,
                Some(internal_descriptor.to_string()),
            )
            .extract_keys()
            .load_wallet_no_persist(changeset)?;

        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => return Err(JsError::new("Failed to load wallet, check the changeset")),
        };

        let client = esplora_client::Builder::new(&url).build_async_with_sleeper()?;

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
            .apply_update_at(update, now)
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    pub async fn sync(&mut self, parallel_requests: usize) -> JsResult<()> {
        let request = self.wallet.start_sync_with_revealed_spks();
        let update = self.client.sync(request, parallel_requests).await?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet.apply_update_at(update, now)?;

        Ok(())
    }
    // --8<-- [end:wallet]

    // --8<-- [start:utils]
    pub fn balance(&self) -> u64 {
        let balance = self.wallet.balance();
        balance.total().to_sat()
    }

    pub fn reveal_next_address(&mut self) -> String {
        let address = self.wallet.reveal_next_address(KeychainKind::External);

        address.to_string()
    }
    // --8<-- [end:utils]

    pub fn peek_address(&mut self, index: u32) -> String {
        let address = self.wallet.peek_address(KeychainKind::External, index);

        address.to_string()
    }

    // --8<-- [start:store]
    pub fn take_staged(&mut self) -> JsResult<String> {
        match self.wallet.take_staged() {
            Some(changeset) => {
                let value = serde_json::to_value(&changeset)?;
                Ok(serde_json::to_string(&value)?)
            }
            None => Ok("null".to_string()),
        }
    }

    pub fn take_merged(&mut self, previous: String) -> JsResult<String> {
        match self.wallet.take_staged() {
            Some(curr_changeset) => {
                let previous_value: Value = serde_json::from_str(&previous)?;
                let mut previous_changeset: ChangeSet = serde_json::from_value(previous_value)?;
                previous_changeset.merge(curr_changeset);
                let final_value = serde_json::to_value(&previous_changeset)?;
                Ok(serde_json::to_string(&final_value)?)
            }
            None => Ok("null".to_string()),
        }
    }
    // --8<-- [end:store]
}
