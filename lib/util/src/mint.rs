use anchor_client::solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

//よく使うトークンをenum型に
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Hash)]
pub enum Mint {
    Usdc,
    Sol,
}

impl Mint {
    pub fn address(&self) -> &str {
        match &self {
            Mint::Usdc => "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            Mint::Sol => "So11111111111111111111111111111111111111112",
        }
    }
    pub fn pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.address()).unwrap()
    }
    pub fn pyth_address(&self) -> &str {
        match &self {
            Mint::Usdc => "Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD",
            Mint::Sol => "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG",
        }
    }
    pub fn pyth_pubkey(&self) -> Pubkey {
        Pubkey::from_str(&self.pyth_address()).unwrap()
    }
}
