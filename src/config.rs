use ethers::types::Address;
use std::{env, str::FromStr};

pub struct Config {
    pub rpc_url: String,
    pub quickswap_router: Address,
    pub sushiswap_router: Address,
    pub weth_address: Address,
    pub usdc_address: Address,
    pub trade_amount_weth: f64,
    pub min_profit_usd: f64,
    pub simulated_gas_cost_usd: f64,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            rpc_url: env::var("RPC_URL")?,
            quickswap_router: Address::from_str(&env::var("QUICKSWAP_ROUTER")?)?,
            sushiswap_router: Address::from_str(&env::var("SUSHISWAP_ROUTER")?)?,
            weth_address: Address::from_str(&env::var("WETH_ADDRESS")?)?,
            usdc_address: Address::from_str(&env::var("USDC_ADDRESS")?)?,
            trade_amount_weth: env::var("TRADE_AMOUNT_WETH")?.parse()?,
            min_profit_usd: env::var("MIN_PROFIT_USD")?.parse()?,
            simulated_gas_cost_usd: env::var("SIMULATED_GAS_COST_USD")?.parse()?,
        })
    }
}
