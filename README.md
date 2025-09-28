# Polygon-Arbitrage-Opportunity-Detector-Bot

## Introduction
This project is a Rust-based bot that detects potential arbitrage opportunities on the Polygon network.
Arbitrage here means buying a token cheaply on one DEX (e.g., SushiSwap) and selling it for a higher price on another DEX (e.g., QuickSwap).

The bot periodically checks prices(10 sec) for a token pair (e.g., WETH/USDC) across two DEXes. If a significant difference exists, it logs the opportunity in a SQLite database along with simulated profit after gas costs.

The profit threshold is set to 5 USDC, if the profit that is calculated after removing the gas cost is greater than the threshold, the bot will detect it as an arbitrage and log this opportunity into the database.
All database entries are visible in the terminal and refresh automatically over time.

## Goals

- Fetch live prices from two Polygon DEXes.
- Detect arbitrage opportunities with profit threshold filtering.
- Simulate profits after subtracting estimated gas costs.
- Store results in a local SQLite database.
- Run continuously with configurable intervals (default: 10s).

## Technology Stack

- Blockchain: Polygon Network
- DEXes: QuickSwap & SushiSwap (Uniswap V2 routers)
- Tokens: WETH, USDC
-Language: Rust
-Database: SQLite

## Project Structure
```
arbitrage_detector_bot/
│── Cargo.toml
│── Cargo.lock
│── README.md
└── src/
    ├── main.rs        # Entry point – connects to Polygon, fetches prices, detects arbitrage
    └── database.rs    # Handles SQLite initialization & logging opportunities
```
---

## Setup Instructions
Prerequisites

- Rust (latest stable) → Install Rust
- SQLite
- Polygon RPC endpoint (default: https://polygon-rpc.com)

Clone & Build
```bash
git clone https://github.com/<your-username>/arbitrage_detector_bot.git
cd arbitrage_detector_bot
cargo build
```

Run
```bash
cargo run
```

## Example Output

QuickSwap: 3995.720294 USDC
SushiSwap: 3946.991025 USDC
Buy Sushi -> Sell Quick | Profit: 48.726466893193404
