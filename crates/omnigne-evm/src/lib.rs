//! EVM integration boundary. Alloy clients, venues and financing are not implemented.
use omnigne_core::{AssetId, ExecutionDomain};

#[derive(Clone, Copy, Debug)]
pub enum EvmNetwork {
    Base,
    Arbitrum,
    Optimism,
}

impl EvmNetwork {
    pub const fn chain_id(self) -> u64 {
        match self {
            Self::Base => 8453,
            Self::Arbitrum => 42161,
            Self::Optimism => 10,
        }
    }

    pub const fn domain(self) -> ExecutionDomain {
        ExecutionDomain::Evm
    }

    pub fn asset(self, contract_address: String) -> AssetId {
        AssetId {
            network: format!("eip155:{}", self.chain_id()),
            address: contract_address,
        }
    }
}
