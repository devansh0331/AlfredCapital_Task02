use ethers::prelude::*;
use dotenv::dotenv;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

mod profit;
use profit::{calculate_profit, ArbitrageDirection};

mod config;
use config::Config;

abigen!(
    IUniswapV2Router,
    r#"[
        function getAmountsOut(uint amountIn, address[] calldata path) external view returns (uint[] memory amounts)
    ]"#
);

async fn fetch_price(
    provider: Arc<Provider<Http>>,
    router_address: Address,
    token_in: Address,
    token_out: Address,
    amount_in: U256,
) -> Result<U256, Box<dyn std::error::Error>> {
    let router = IUniswapV2Router::new(router_address, provider.clone());
    let path = vec![token_in, token_out];
    let amounts_out = router.get_amounts_out(amount_in, path).call().await?;
    Ok(*amounts_out.last().unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Configuration
    let cfg = Config::from_env()?;

    // Connect provider
    let provider = Arc::new(Provider::<Http>::try_from(cfg.rpc_url.clone())?);

    // Amount in (18 decimals for WETH)
    let amount_in: U256 = ethers::utils::parse_units(cfg.trade_amount_weth, 18)?.into();

    println!("Arbitrage bot started. Monitoring QuickSwap and SushiSwap every 5 seconds...\n");

    loop {
        // Fetch prices
        let price_quick = fetch_price(
            provider.clone(),
            cfg.quickswap_router,
            cfg.weth_address,
            cfg.usdc_address,
            amount_in,
        )
        .await?;
        let price_sushi = fetch_price(
            provider.clone(),
            cfg.sushiswap_router,
            cfg.weth_address,
            cfg.usdc_address,
            amount_in,
        )
        .await?;

        // Format prices
        let price_quick_usdc = ethers::utils::format_units(price_quick, 6)?.parse::<f64>()?;
        let price_sushi_usdc = ethers::utils::format_units(price_sushi, 6)?.parse::<f64>()?;

        println!(
            "QuickSwap: {:.6} USDC for {} WETH",
            price_quick_usdc, cfg.trade_amount_weth
        );
        println!(
            "SushiSwap: {:.6} USDC for {} WETH",
            price_sushi_usdc, cfg.trade_amount_weth
        );

        // Arbitrage Detection
        let result = calculate_profit(price_quick_usdc, price_sushi_usdc, cfg.simulated_gas_cost_usd);

        match result.direction {
            ArbitrageDirection::SushiSwapToQuickSwap => { 
                if result.profit_usd >= cfg.min_profit_usd {
                    println!(
                        "Arbitrage: Buy on SushiSwap, Sell on QuickSwap | Net Profit: ${:.2}",
                        result.profit_usd
                    );
                } else {
                    println!(
                        "Opportunity detected but below min profit threshold (${:.2})",
                        cfg.min_profit_usd
                    );
                }
            }
            ArbitrageDirection::QuickSwapToSushiSwap => {
                if result.profit_usd >= cfg.min_profit_usd {
                    println!(
                        "Arbitrage: Buy on QuickSwap, Sell on SushiSwap | Net Profit: ${:.2}",
                        result.profit_usd
                    );
                } else {
                    println!(
                        "Opportunity detected but below min profit threshold (${:.2})",
                        cfg.min_profit_usd
                    );
                }
            }
            ArbitrageDirection::NoArbitrage => {
                println!(
                    "No arbitrage opportunity after gas cost (${:.2})",
                    cfg.simulated_gas_cost_usd
                );
            }
        }

        println!("---\n");

        // Wait before next check
        sleep(Duration::from_secs(5)).await;
    }
}
