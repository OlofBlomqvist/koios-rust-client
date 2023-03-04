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
    let mut cfg = Configuration::new();
    cfg.base_path = "https://preprod.koios.rest/api/v0".into();
    let tip = apis::network_api::tip_get(&cfg).await.unwrap();
    let tip = tip.get(0).unwrap();
    let params = apis::epoch_api::epoch_params_get(&cfg, Some(&tip.epoch_no.unwrap().to_string())).await.unwrap();
    let params_for_this_epoch = params.get(0).unwrap();
    assert!(params_for_this_epoch.coins_per_utxo_size.is_some());
}

#[cfg(test)]
#[tokio::test]
async fn tx_info() {

    let mut cfg = Configuration::new();
    cfg.base_path = "https://preprod.koios.rest/api/v0".into();

    let result = apis::transactions_api::tx_info_post(&cfg, Some(models::TxInfoPostRequest { 
        _tx_hashes: vec![
            "0cea71bb08ee44e4cd1f68748d292281b525d56888456a0ea1e45a15ef516b40".into(),
            "d1e488d70850fc36ef6971a65d1f45e2a7a433e4080e4c5c580d0b23094a955e".into(),
            "c41e732419c97613a81842dbd6ac2ce5d022d7cb2a19f1c8b459e81d6d001041".into(),
            "1aef839aa16a483d8d0e42a0899b1a75d6c7025df79368d3059b35756d2be35a".into()
    ] })).await;

    match result {
        Err(e) => panic!("{:?}",e),
        Ok(r) => {
            assert!(r.len()==4)
        }
    }

}