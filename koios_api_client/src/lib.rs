use apis::configuration::Configuration;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

pub mod apis;
pub mod models;


#[cfg(test)]
#[tokio::test]
async fn epoch_params() {
    let cfg = Configuration::new();
    let tip = apis::network_api::tip_get(&cfg).await.unwrap();
    let tip = tip.get(0).unwrap();
    let params = apis::epoch_api::epoch_params_get(&cfg, Some(&tip.epoch_no.unwrap().to_string())).await.unwrap();
    let params_for_this_epoch = params.get(0).unwrap();
    assert!(params_for_this_epoch.coins_per_utxo_size.is_some())
}