use solana_sdk::signer::keypair::Keypair;
use anyhow::Result;

const SECRET_KEY: &str = "your secret key";

#[tokio::main]
async fn main() -> Result<()> {
    // String型の秘密鍵を読み込み
    let payer = Keypair::from_base58_string(SECRET_KEY);

    //[u8; 64]型の秘密鍵を表示
    let bytes = payer.to_bytes();
    println!("{:?}", bytes);
    Ok(())
}