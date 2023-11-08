use std::{error::Error, str::FromStr, sync::Arc};
use std::env;

use ethers::{
    providers::{Http, Provider},
    types::H160,
};

use cfmms::{
    dex::{Dex, DexVariant},
    sync,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let key = "ETHEREUM_MAINNET_ENDPOINT";
    env::set_var(key, "https://mainnet.infura.io/v3/7d89bf1eb27e4787afbe6abcd94acf73");
    //Add rpc endpoint here:
    let rpc_endpoint = std::env::var("ETHEREUM_MAINNET_ENDPOINT")
        .expect("Could not get ETHEREUM_MAINNET_ENDPOINT");
    let provider = Arc::new(Provider::<Http>::try_from(rpc_endpoint).unwrap());

    let dexes = vec![
        //UniswapV2
        Dex::new(
            H160::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap(),
            DexVariant::UniswapV2,
            2638438,
            Some(300),
        ),
        //Add Sushiswap
        Dex::new(
            H160::from_str("0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac").unwrap(),
            DexVariant::UniswapV2,
            10794229,
            Some(300),
        ),
        //Add UniswapV3
        Dex::new(
            H160::from_str("0x1F98431c8aD98523631AE4a59f267346ea31F984").unwrap(),
            DexVariant::UniswapV3,
            12369621,
            None,
        ),
    ];

    //Sync pairs
    sync::sync_pairs(dexes, provider, None).await?;

    Ok(())
}
