




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EpochParamsInner {
    /// Epoch number
    #[serde(rename = "epoch_no", skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<i128>,
    /// The 'a' parameter to calculate the minimum transaction fee
    #[serde(rename = "min_fee_a", default, skip_serializing_if = "Option::is_none")]
    pub min_fee_a: Option<i128>,
    /// The 'b' parameter to calculate the minimum transaction fee
    #[serde(rename = "min_fee_b", default, skip_serializing_if = "Option::is_none")]
    pub min_fee_b: Option<i128>,
    /// The maximum block size (in bytes)
    #[serde(rename = "max_block_size", default, skip_serializing_if = "Option::is_none")]
    pub max_block_size: Option<i128>,
    /// The maximum transaction size (in bytes)
    #[serde(rename = "max_tx_size", default, skip_serializing_if = "Option::is_none")]
    pub max_tx_size: Option<i128>,
    /// The maximum block header size (in bytes)
    #[serde(rename = "max_bh_size", default, skip_serializing_if = "Option::is_none")]
    pub max_bh_size: Option<i128>,
    /// The amount (in lovelace) required for a deposit to register a stake address
    #[serde(rename = "key_deposit", default, skip_serializing_if = "Option::is_none")]
    pub key_deposit: Option<String>,
    /// The amount (in lovelace) required for a deposit to register a stake pool
    #[serde(rename = "pool_deposit", default, skip_serializing_if = "Option::is_none")]
    pub pool_deposit: Option<String>,
    /// The maximum number of epochs in the future that a pool retirement is allowed to be scheduled for
    #[serde(rename = "max_epoch", default, skip_serializing_if = "Option::is_none")]
    pub max_epoch: Option<i128>,
    /// The optimal number of stake pools
    #[serde(rename = "optimal_pool_count", default, skip_serializing_if = "Option::is_none")]
    pub optimal_pool_count: Option<i128>,
    /// The pledge influence on pool rewards
    #[serde(rename = "influence", default, skip_serializing_if = "Option::is_none")]
    pub influence: Option<f64>,
    /// The monetary expansion rate
    #[serde(rename = "monetary_expand_rate", default, skip_serializing_if = "Option::is_none")]
    pub monetary_expand_rate: Option<f64>,
    /// The treasury growth rate
    #[serde(rename = "treasury_growth_rate", default, skip_serializing_if = "Option::is_none")]
    pub treasury_growth_rate: Option<f64>,
    /// The decentralisation parameter (1 fully centralised, 0 fully decentralised)
    #[serde(rename = "decentralisation", default, skip_serializing_if = "Option::is_none")]
    pub decentralisation: Option<f64>,
    /// The hash of 32-byte string of extra random-ness added into the protocol's entropy pool
    #[serde(rename = "extra_entropy", default, skip_serializing_if = "Option::is_none")]
    pub extra_entropy: Option<String>,
    /// The protocol major version
    #[serde(rename = "protocol_major", default, skip_serializing_if = "Option::is_none")]
    pub protocol_major: Option<i128>,
    /// The protocol minor version
    #[serde(rename = "protocol_minor", default, skip_serializing_if = "Option::is_none")]
    pub protocol_minor: Option<i128>,
    /// The minimum value of a UTxO entry
    #[serde(rename = "min_utxo_value", default, skip_serializing_if = "Option::is_none")]
    pub min_utxo_value: Option<String>,
    /// The minimum pool cost
    #[serde(rename = "min_pool_cost", default, skip_serializing_if = "Option::is_none")]
    pub min_pool_cost: Option<String>,
    /// The nonce value for this epoch
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// The hash of the first block where these parameters are valid
    #[serde(rename = "block_hash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// The per language cost models
    #[serde(rename = "cost_models", default, skip_serializing_if = "Option::is_none")]
    pub cost_models: Option<String>,
    /// The per word cost of script memory usage
    #[serde(rename = "price_mem", default, skip_serializing_if = "Option::is_none")]
    pub price_mem: Option<f64>,
    /// The cost of script execution step usage
    #[serde(rename = "price_step", default, skip_serializing_if = "Option::is_none")]
    pub price_step: Option<f64>,
    /// The maximum number of execution memory allowed to be used in a single transaction
    #[serde(rename = "max_tx_ex_mem", default, skip_serializing_if = "Option::is_none")]
    pub max_tx_ex_mem: Option<f32>,
    /// The maximum number of execution steps allowed to be used in a single transaction
    #[serde(rename = "max_tx_ex_steps", default, skip_serializing_if = "Option::is_none")]
    pub max_tx_ex_steps: Option<f32>,
    /// The maximum number of execution memory allowed to be used in a single block
    #[serde(rename = "max_block_ex_mem", default, skip_serializing_if = "Option::is_none")]
    pub max_block_ex_mem: Option<f32>,
    /// The maximum number of execution steps allowed to be used in a single block
    #[serde(rename = "max_block_ex_steps", default, skip_serializing_if = "Option::is_none")]
    pub max_block_ex_steps: Option<f32>,
    /// The maximum Val size
    #[serde(rename = "max_val_size", default, skip_serializing_if = "Option::is_none")]
    pub max_val_size: Option<f32>,
    /// The percentage of the tx fee which must be provided as collateral when including non-native scripts
    #[serde(rename = "collateral_percent", default, skip_serializing_if = "Option::is_none")]
    pub collateral_percent: Option<i128>,
    /// The maximum number of collateral inputs allowed in a transaction
    #[serde(rename = "max_collateral_inputs", default, skip_serializing_if = "Option::is_none")]
    pub max_collateral_inputs: Option<i128>,
    /// The cost per UTxO size
    #[serde(rename = "coins_per_utxo_size", default, skip_serializing_if = "Option::is_none")]
    pub coins_per_utxo_size: Option<String>,
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


