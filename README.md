# Task 02: Polygon  Arbitrage Opportunity Detector Bot

## Introduction
This project is a **Rust-based arbitrage detection bot** for the **Polygon network**.  
The bot simulates arbitrage opportunities between **QuickSwap** and **SushiSwap** by comparing token prices (e.g., WETH/USDC) and calculating potential profit after subtracting a simulated gas cost.

Arbitrage means **buying a token cheaply on one exchange and selling it for a higher price on another**.

---

## Goal
- Periodically fetch token pair prices (WETH/USDC) from QuickSwap and SushiSwap on Polygon.  
- Detect profitable arbitrage opportunities.  
- Log opportunities with simulated net profit.  

---

## Key Metrics
- **Simulated Arbitrage Profit** → estimated profit in USDC after subtracting simulated gas cost.

---

## Deliverables
1. **Multi-DEX Price Fetching** → queries Uniswap V2-style routers (QuickSwap & SushiSwap).  
2. **Arbitrage Detection** → identifies price differences above a threshold.  
3. **Simulated Profit Calculation** → computes profit for fixed trade size minus gas.  
4. **Configuration Management** → centralized `.env` configuration via `Config` struct.  
5. **Presentation** → logic flow, schema, and system architecture (this document).  

---

## Technology Stack
- **Blockchain**: Polygon Network  
- **DEXes**: QuickSwap, SushiSwap (Uniswap V2-style)  
- **Tokens**: WETH, USDC (extendable)  
- **Language**: Rust  
- **Libraries**: `ethers-rs`, `tokio`, `dotenv`  
 ---

## Setup Instructions

### 1. Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)

- Git



### 2. Clone Repository
```bash
git clone https://github.com/devansh0331/AlfredCapital_Task02.git
cd AlfredCapital_Task02
```


### 3. Build Project
```bash
cargo build
```


### 4. Run
```bash
cargo run
```

---


## Logic Flow
```
START
 │
 │── Load configuration from .env (RPC URL, QuickSwap, SushiSwap, token addresses, thresholds)
 │
 │── Connect to Polygon RPC provider
 │
 │── Fetch simulated prices from QuickSwap and SushiSwap
 │
 │── Convert prices into human-readable floats
 │
 │── Arbitrage detection:
 │      • Compare QuickSwap vs SushiSwap
 │      • Subtract gas cost
 │      • Check if profit > threshold
 │
 │── Log result (arbitrage direction + profit OR no opportunity)
 │
END
```

---

## Database Schema
If logging opportunities into a database:

**Table: `arbitrage_opportunities`**
| Column            | Type      | Description |
|-------------------|----------|-------------|
| `id`              | INTEGER (PK) | Unique row ID |
| `timestamp`       | DATETIME  | When the check was performed |
| `dex_buy`         | TEXT      | DEX where you buy (QuickSwap/SushiSwap) |
| `dex_sell`        | TEXT      | DEX where you sell (QuickSwap/SushiSwap) |
| `token_pair`      | TEXT      | e.g., WETH/USDC |
| `trade_amount`    | REAL      | Trade size in WETH |
| `price_buy`       | REAL      | Price on buy DEX |
| `price_sell`      | REAL      | Price on sell DEX |
| `gas_cost_usd`    | REAL      | Simulated gas cost |
| `net_profit_usd`  | REAL      | Net profit |
| `status`          | TEXT      | profitable / not_profitable |

---

## System Architecture
```
                 ┌────────────────────────────┐
                 │         .env Config        │
                 │  (RPC, QuickSwap, SushiSwap│
                 │   Tokens, Thresholds, etc.)│
                 └─────────────┬──────────────┘
                               │
                     Load into Config Struct
                               │
 ┌─────────────────────────────▼─────────────────────────────┐
 │                     Arbitrage Bot (Rust)                  │
 │                                                           │
 │  ┌───────────────┐     ┌────────────────┐                 │
 │  │  DEX Module   │◄───►│ Polygon RPC/API│                 │
 │  │ (fetch_price) │     └────────────────┘                 │
 │  └───────────────┘                                        │
 │          │                                                │
 │          ▼                                                │
 │   ┌───────────────────┐   ┌─────────────────────┐         │
 │   │ Profit Calculation│   │  Config Management  │         │
 │   │ (price diff, gas) │   │ (centralized struct)│         │
 │   └───────────────────┘   └─────────────────────┘         │
 │          │                                                │
 │          ▼                                                │
 │   ┌─────────────────────────────────────────────┐         │
 │   │ Logging / Persistence                       │         │
 │   └─────────────────────────────────────────────┘         │
 │                                                           │
 └───────────────────────────────────────────────────────────┘
```
