// Here DEXA is QUICKSWAP and DEXB is SUSHISWAP
pub enum ArbitrageDirection {
    QuickSwapToSushiSwap,
    SushiSwapToQuickSwap,
    NoArbitrage,
}

pub struct ProfitResult {
    pub profit_usd: f64,
    pub direction: ArbitrageDirection,
}

// Calculate arbitrage profit considering gas cost
pub fn calculate_profit(
    price_a: f64,
    price_b: f64,
    gas_cost_usd: f64,
) -> ProfitResult {
    if price_a > price_b {
        let profit = price_a - price_b - gas_cost_usd;
        // println!("Debug: Price A: {}, Price B: {}, Gas Cost: {}, Profit: {}", price_a, price_b, gas_cost_usd, profit);
        if profit > 0.0 {
            return ProfitResult {
                profit_usd: profit,
                direction: ArbitrageDirection::SushiSwapToQuickSwap,
            };
        }
    } else if price_b > price_a {
        let profit = price_b - price_a - gas_cost_usd;
        //  println!("Debug: Price A: {}, Price B: {}, Gas Cost: {}, Profit: {}", price_a, price_b, gas_cost_usd, profit);
        if profit > 0.0 {
            return ProfitResult {
                profit_usd: profit,
                direction: ArbitrageDirection::QuickSwapToSushiSwap,
            };
        }
    }

    ProfitResult {
        profit_usd: 0.0,
        direction: ArbitrageDirection::NoArbitrage,
    }
}
