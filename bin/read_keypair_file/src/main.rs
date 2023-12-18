use solana_sdk::{signature::Signer, signer::keypair::read_keypair_file};
use anyhow::Result;
use util::const_str::PAYER;

// String型の秘密鍵を使う場合
// const SECRET_KEY: &str = "your secret key";

#[tokio::main]
async fn main() -> Result<()> {
    //[u8型の秘密鍵をキーペアファイルから読み込み]
    let payer = read_keypair_file(&*shellexpand::tilde(PAYER))
        .expect("cannot read keypair");

    // String型の秘密鍵を読み込み
    // let payer = Keypair::from_base58_string(SECRETKEY);

    // pubkeyを表示
    let base58 = payer.pubkey();
    println!("pubkey:");
    println!("{:?}", base58);
    println!();

    //[u8; 64]型の秘密鍵を表示
    let bytes = payer.to_bytes();
    println!("secret_key[u8; 64]:");
    println!("{:?}", bytes);
    println!();

    //String型の秘密鍵を表示
    let base58 = payer.to_base58_string();
    println!("secret_key[String]:");
    println!("{}", base58);
    println!();
    Ok(())
}