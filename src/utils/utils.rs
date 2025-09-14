use anyhow::Result;
use std::fmt::Display;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpportunityStatus {
    Succeeded,          // Transaction was successful
    PartiallySucceeded, // Transaction was partially successful
    Reverted,           // Transaction reverted
    Error,              // Transaction failed with an error
    Skipped,            // Transaction was skipped (e.g. no profit)
    None,
}

impl Display for OpportunityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_network_name(network_id: u64) -> Result<String> {
    match network_id {
        1 => Ok("eth".to_string()),
        8453 => Ok("base".to_string()),
        137 => Ok("polygon".to_string()),
        1116 => Ok("core".to_string()),
        14 => Ok("flare".to_string()),
        43114 => Ok("avax".to_string()),
        1514 => Ok("story".to_string()),
        252 => Ok("fraxtal".to_string()),
        5000 => Ok("mantle".to_string()),
        2222 => Ok("kava".to_string()),
        80094 => Ok("bera".to_string()),
        747474 => Ok("katana".to_string()),
        369 => Ok("pulsechain".to_string()),
        4689 => Ok("iotx".to_string()),
        7000 => Ok("zetachain".to_string()),
        1088 => Ok("metis".to_string()),
        50 => Ok("xdc".to_string()),
        543210 => Ok("zero-network".to_string()),
        88888 => Ok("chiliz-chain".to_string()),
        1135 => Ok("lisk".to_string()),
        11350 => Ok("rootstock".to_string()),
        56 => Ok("bsc".to_string()),
        42793 => Ok("etherlink".to_string()),
        177 => Ok("hashkey".to_string()),
        5464 => Ok("saga".to_string()),
        1329 => Ok("sei-evm".to_string()),
        5031 => Ok("somnia".to_string()),

        _ => Err(anyhow::anyhow!("Unknown network id: {}", network_id)),
    }
}
