use anchor_client::{
    anchor_lang::AccountDeserialize,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        pubkey::Pubkey, signature::read_keypair_file, signature::Signer, signer::keypair::Keypair,
    },
    Cluster,
};
use anchor_spl::{associated_token::get_associated_token_address, token::TokenAccount};
use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};
use util::mint::Mint;
use util::const_str::PAYER;

// const SECRET_KEY: &str = "your secret key"; //シークレットキーを直で読み込む場合はこちら

//JupiterのToken APIからTokenデータを取得
pub async fn get_token_info() -> Result<HashMap<Pubkey, TokenInfo>> {
    let client = reqwest::Client::new();
    let mut token_info: HashMap<Pubkey, TokenInfo> = HashMap::new();
    let url = "https://token.jup.ag/strict"; //strict or all
    let response = client.get(url).send().await?;
    let mut json = response.json::<Vec<TokenInfo>>().await?;
    for j in json.iter_mut() {
        token_info.insert(j.pubkey(), j.clone());
    }
    Ok(token_info)
}

//Jupiter Token APIから取得したデータを格納する
#[derive(Debug, Default, Clone, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
}

impl TokenInfo {
    pub fn pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.address).unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    //ファイルからキーペアを取得
    let payer = read_keypair_file(&*shellexpand::tilde(PAYER))
        .expect("cannot read keypair");
    // let payer = Keypair::from_base58_string(SECRET_KEY);  //シークレットキーを直で読み込む場合はこちら
    let rpc_client = RpcClient::new(Cluster::Mainnet.url().to_string());

    //JupiterのToken APIからToken情報を取得
    let tokens_info = get_token_info().await?;
    // println!("{:?}", token_info);

    //ネイティブSOLのToken情報を取得
    let sol_info = tokens_info.get(&Mint::Sol.pubkey()).unwrap();
    //SOLの残高を取得
    let sol_amount = rpc_client.get_balance(&payer.pubkey()).await?;
    let sol_ui_amount = spl_token::amount_to_ui_amount(sol_amount, sol_info.decimals);
    println!("{:?} {}", sol_ui_amount, sol_info.symbol);

    //wSOLとUSDCの残高を取得
    for mint in [Mint::Sol, Mint::Usdc] {
        //Token情報を取得
        let token_info = tokens_info.get(&mint.pubkey()).unwrap();
        // println!("token_info: {:?}", token_info);

        //ata(自分のウォレットのwSOL, USDCアカウント）のpubkeyを取得
        let ata = get_associated_token_address(&payer.pubkey(), &mint.pubkey());
        let data = rpc_client.get_account_data(&ata).await?;
        let token = TokenAccount::try_deserialize(&mut &data[..])?;
        // println!("{:?}", token);
        let token_amount = spl_token::amount_to_ui_amount(token.amount, token_info.decimals);
        println!("{:?} {}", token_amount, token_info.symbol);
    }

    Ok(())
}
