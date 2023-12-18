use solana_client::rpc_client::RpcClient;
use anyhow::Result;
use pyth_sdk_solana::load_price_feed_from_account;
use std::time::{SystemTime, UNIX_EPOCH};
use util::mint::Mint;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https:/api.mainnet-beta.solana.com";
    let clnt = RpcClient::new(url.to_string());
    let sol_price_key = Mint::Sol.pyth_pubkey();

    let mut sol_price_account = clnt.get_account(&sol_price_key).unwrap();
    let sol_price_feed =
        load_price_feed_from_account(&sol_price_key, &mut sol_price_account).unwrap();

    println!(".....SOL/USD.....");

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let maybe_price = sol_price_feed.get_price_no_older_than(current_time, 60);
    match maybe_price {
        Some(p) => {
            let price = p.price as f64 / 10u64.pow((-p.expo).try_into().unwrap()) as f64;
            println!("price ........... {}", price);
        }
        None => {
            println!("price ........... unavailable");
        }
    }

    let maybe_ema_price = sol_price_feed.get_ema_price_no_older_than(current_time, 60);
    match maybe_ema_price {
        Some(ema_price) => {
            let ema =
                ema_price.price as f64 / 10u64.pow((-ema_price.expo).try_into().unwrap()) as f64;
            println!("ema_price ....... {}", ema);
        }
        None => {
            println!("ema_price ....... unavailable");
        }
    }
    println!();
    Ok(())
}
