//! Unsigned intent terms only. Shape validation is not signature verification.
//! No API, custody, execution or external intent-network integration is enabled.
use omnigne_core::AssetId;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntentTerms {
    pub settler: String,
    pub owner: String,
    pub recipient: String,
    pub input: AssetId,
    pub output: AssetId,
    pub max_input: u64,
    pub min_output: u64,
    pub nonce: String,
    pub expires_at_unix: u64,
    pub allow_partial_fill: bool,
}

#[derive(Debug, Error)]
#[error("invalid or expired unsigned intent terms")]
pub struct InvalidIntentTerms;

impl IntentTerms {
    pub fn validate_shape(&self, now: u64) -> Result<(), InvalidIntentTerms> {
        if self.settler.is_empty()
            || self.owner.is_empty()
            || self.recipient.is_empty()
            || self.input.network.is_empty()
            || self.input.address.is_empty()
            || self.output.address.is_empty()
            || self.input.network != self.output.network
            || self.input == self.output
            || self.max_input == 0
            || self.min_output == 0
            || self.nonce.is_empty()
            || self.expires_at_unix <= now
        {
            return Err(InvalidIntentTerms);
        }
        Ok(())
    }
}
