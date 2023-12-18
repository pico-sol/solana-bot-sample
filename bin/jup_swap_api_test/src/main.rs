use anchor_client::{
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig},
    solana_sdk::{
        commitment_config::CommitmentLevel, signature::read_keypair_file, signature::Signer,
        signer::keypair::Keypair, transaction::VersionedTransaction,
    },
};
use anyhow::Result;
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use std::env;
use util::const_str::PAYER;
use util::mint::Mint;

// const PAYER: &str = "~/.config/solana/id.json";
// String型の秘密鍵を使う場合
// const SECRET_KEY: &str = "your secret key";

#[tokio::main]
async fn main() -> Result<()> {
    let payer = read_keypair_file(&*shellexpand::tilde(PAYER)).expect("cannot read keypair");
    // String型の秘密鍵を読み込み
    // let payer = Keypair::from_base58_string(SECRET_KEY);

    let api_base_url = env::var("API_BASE_URL").unwrap_or("https://quote-api.jup.ag/v6".into());
    println!("Using base url: {}", api_base_url);

    let jupiter_swap_api_client = JupiterSwapApiClient::new(api_base_url);

    // 0.01 SOLをスリッページ50bps(=0.5％)でUSDCにswapします
    let quote_request = QuoteRequest {
        amount: 1_000_000,
        input_mint: Mint::Sol.pubkey(),
        output_mint: Mint::Usdc.pubkey(),
        slippage_bps: 50,
        ..QuoteRequest::default()
    };

    // GET /quote
    let quote_response = jupiter_swap_api_client.quote(&quote_request).await.unwrap();
    println!("{quote_response:#?}");

    // POST /swap
    let swap_response = jupiter_swap_api_client
        .swap(&SwapRequest {
            user_public_key: payer.pubkey(),
            quote_response: quote_response.clone(),
            config: TransactionConfig::default(),
        })
        .await?;

    // println!("Raw tx len: {}", swap_response.swap_transaction.len());

    let versioned_transaction: VersionedTransaction =
        bincode::deserialize(&swap_response.swap_transaction)?;

    // println!("message: {:?}", versioned_transaction.message);

    let signed_versioned_transaction =
        VersionedTransaction::try_new(versioned_transaction.message, &[&payer])?;

    // send with rpc client...
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".into());

    let config = RpcSendTransactionConfig {
        preflight_commitment: Some(CommitmentLevel::Processed),
        ..RpcSendTransactionConfig::default()
    };

    // 実際にtxを実行してしまうので念のためコメントアウトしてあります。適宜はずしてください
    let signature = rpc_client
        .send_transaction_with_config(&signed_versioned_transaction, config)
        .await?;
    println!("send_tx: {}", signature);
    Ok(())
}
