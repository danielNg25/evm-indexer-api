pub mod erc4626_standard;
pub mod verio_ip;
use serde::{Deserialize, Serialize};

pub use erc4626_standard::ERC4626Standard;
pub use verio_ip::VerioIP;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERC4626Pool {
    VerioIP,
}
