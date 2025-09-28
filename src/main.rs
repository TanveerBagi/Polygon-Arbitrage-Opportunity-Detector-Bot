mod database;

use ethers::prelude::*;
use anyhow::Result;
use std::sync::Arc;
use tokio::time::{sleep, Duration};


abigen!(
    UniswapV2Router,
    r#"[
        function getAmountsOut(uint amountIn, address[] memory path) public view returns (uint[] memory amounts)
    ]"#,
);

const QUICKSWAP_ROUTER: &str = "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff";
const SUSHISWAP_ROUTER: &str = "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506";

const WETH: &str = "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619";
const USDC: &str = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174";



#[tokio::main]
async fn main() -> Result<()> {

    let provider = Provider::<Http>::try_from("https://polygon-rpc.com")?;
    let client = Arc::new(provider);

    std::fs::create_dir_all("data")?;
    let conn = database::init_db()?;

    let quickswap = UniswapV2Router::new(QUICKSWAP_ROUTER.parse::<Address>()?, client.clone());
    let sushiswap = UniswapV2Router::new(SUSHISWAP_ROUTER.parse::<Address>()?, client.clone());


    let path: Vec<Address> = vec![
        WETH.parse()?,
        USDC.parse()?,
    ];


    loop {
        let amount_in = U256::exp10(18);

        let quick_price = quickswap.get_amounts_out(amount_in, path.clone()).call().await?;
        let quick_out = quick_price[1];
        let quick_out_inusd = (quick_out.as_u128() as f64) / 1e6;

        let sushi_price = sushiswap.get_amounts_out(amount_in, path.clone()).call().await?;
        let sushi_out = sushi_price[1];
        let sushi_out_inusd = (sushi_out.as_u128() as f64) / 1e6;

        println!("QuickSwap: {} USDC", quick_out_inusd);
        println!("SushiSwap: {} USDC", sushi_out_inusd);

        let gas_price = client.get_gas_price().await?; //in wei
        let gas_used: u64 = 200_000;

        let gas_cost_wei = gas_price * gas_used;
        let gas_cost_matic = gas_cost_wei.as_u128() as f64 / 1e18;

        let matic_usd_price = 0.5;
        let gas_cost_usd = gas_cost_matic * matic_usd_price;

        if quick_out > sushi_out {
            let profit = quick_out - sushi_out;
            let mut profit_in_usd = (profit.as_u128() as f64) / 1e6;
            profit_in_usd -= gas_cost_usd;
            if profit_in_usd > 5.0{
                println!("Buy Sushi -> Sell Quick | Profit: {}", profit_in_usd);
                database::save_opportunity(&conn, "WETH/USDC", "SushiSwap", "QuickSwap", profit_in_usd)?;
            }


        } else if sushi_out > quick_out {
            let profit = sushi_out - quick_out;
            let mut profit_in_usd = (profit.as_u128() as f64) / 1e6;
            profit_in_usd -= gas_cost_usd;
            if profit_in_usd > 5.0{
                println!("Buy Quick -> Sell Sushi | Profit: {}", profit_in_usd);
                database::save_opportunity(&conn, "WETH/USDC", "QuickSwap", "SushiSwap", profit_in_usd)?;
            }


        } else {
            println!("No arbitrage opportunity found.");
        }
        //conn.execute("DROP TABLE IF EXISTS opportunities", [])?;
        println!("|  ID   |       Time         |  Token Pair |  Buy DEX  |  Sell DEX  |      Profit      |");
        database::show_opportunities(&conn)?;

        sleep(Duration::from_secs(10)).await;
        println!("|---------------------------------------------------------------------------------------------------------------------|");


    }

}
