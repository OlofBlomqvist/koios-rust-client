




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EpochParamsInner {
    /// Epoch number
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i32>,
    /// The 'a' parameter to calculate the minimum transaction fee
    #[serde(rename = "min_fee_a", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub min_fee_a: Option<Option<i32>>,
    /// The 'b' parameter to calculate the minimum transaction fee
    #[serde(rename = "min_fee_b", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub min_fee_b: Option<Option<i32>>,
    /// The maximum block size (in bytes)
    #[serde(rename = "max_block_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_block_size: Option<Option<i32>>,
    /// The maximum transaction size (in bytes)
    #[serde(rename = "max_tx_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_tx_size: Option<Option<i32>>,
    /// The maximum block header size (in bytes)
    #[serde(rename = "max_bh_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_bh_size: Option<Option<i32>>,
    /// The amount (in lovelace) required for a deposit to register a stake address
    #[serde(rename = "key_deposit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub key_deposit: Option<Option<String>>,
    /// The amount (in lovelace) required for a deposit to register a stake pool
    #[serde(rename = "pool_deposit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pool_deposit: Option<Option<String>>,
    /// The maximum number of epochs in the future that a pool retirement is allowed to be scheduled for
    #[serde(rename = "max_epoch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_epoch: Option<Option<i32>>,
    /// The optimal number of stake pools
    #[serde(rename = "optimal_pool_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub optimal_pool_count: Option<Option<i32>>,
    /// The pledge influence on pool rewards
    #[serde(rename = "influence", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub influence: Option<Option<f64>>,
    /// The monetary expansion rate
    #[serde(rename = "monetary_expand_rate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub monetary_expand_rate: Option<Option<f64>>,
    /// The treasury growth rate
    #[serde(rename = "treasury_growth_rate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub treasury_growth_rate: Option<Option<f64>>,
    /// The decentralisation parameter (1 fully centralised, 0 fully decentralised)
    #[serde(rename = "decentralisation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub decentralisation: Option<Option<f64>>,
    /// The hash of 32-byte string of extra random-ness added into the protocol's entropy pool
    #[serde(rename = "extra_entropy", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub extra_entropy: Option<Option<String>>,
    /// The protocol major version
    #[serde(rename = "protocol_major", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub protocol_major: Option<Option<i32>>,
    /// The protocol minor version
    #[serde(rename = "protocol_minor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub protocol_minor: Option<Option<i32>>,
    /// The minimum value of a UTxO entry
    #[serde(rename = "min_utxo_value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub min_utxo_value: Option<Option<String>>,
    /// The minimum pool cost
    #[serde(rename = "min_pool_cost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub min_pool_cost: Option<Option<String>>,
    /// The nonce value for this epoch
    #[serde(rename = "nonce", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Option<String>>,
    /// The hash of the first block where these parameters are valid
    #[serde(rename = "block_hash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// The per language cost models
    #[serde(rename = "cost_models", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cost_models: Option<Option<String>>,
    /// The per word cost of script memory usage
    #[serde(rename = "price_mem", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price_mem: Option<Option<f64>>,
    /// The cost of script execution step usage
    #[serde(rename = "price_step", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price_step: Option<Option<f64>>,
    /// The maximum number of execution memory allowed to be used in a single transaction
    #[serde(rename = "max_tx_ex_mem", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_tx_ex_mem: Option<Option<f32>>,
    /// The maximum number of execution steps allowed to be used in a single transaction
    #[serde(rename = "max_tx_ex_steps", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_tx_ex_steps: Option<Option<f32>>,
    /// The maximum number of execution memory allowed to be used in a single block
    #[serde(rename = "max_block_ex_mem", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_block_ex_mem: Option<Option<f32>>,
    /// The maximum number of execution steps allowed to be used in a single block
    #[serde(rename = "max_block_ex_steps", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_block_ex_steps: Option<Option<f32>>,
    /// The maximum Val size
    #[serde(rename = "max_val_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_val_size: Option<Option<f32>>,
    /// The percentage of the tx fee which must be provided as collateral when including non-native scripts
    #[serde(rename = "collateral_percent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub collateral_percent: Option<Option<i32>>,
    /// The maximum number of collateral inputs allowed in a transaction
    #[serde(rename = "max_collateral_inputs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_collateral_inputs: Option<Option<i32>>,
    /// The cost per UTxO size
    #[serde(rename = "coins_per_utxo_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub coins_per_utxo_size: Option<Option<String>>,
}

impl EpochParamsInner {
    pub fn new() -> EpochParamsInner {
        EpochParamsInner {
            epoch_no: None,
            min_fee_a: None,
            min_fee_b: None,
            max_block_size: None,
            max_tx_size: None,
            max_bh_size: None,
            key_deposit: None,
            pool_deposit: None,
            max_epoch: None,
            optimal_pool_count: None,
            influence: None,
            monetary_expand_rate: None,
            treasury_growth_rate: None,
            decentralisation: None,
            extra_entropy: None,
            protocol_major: None,
            protocol_minor: None,
            min_utxo_value: None,
            min_pool_cost: None,
            nonce: None,
            block_hash: None,
            cost_models: None,
            price_mem: None,
            price_step: None,
            max_tx_ex_mem: None,
            max_tx_ex_steps: None,
            max_block_ex_mem: None,
            max_block_ex_steps: None,
            max_val_size: None,
            collateral_percent: None,
            max_collateral_inputs: None,
            coins_per_utxo_size: None,
        }
    }
}


