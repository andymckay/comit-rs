#[macro_use]
mod transition_save;

pub mod alice;
pub mod bitcoin;
pub mod bob;
pub mod ethereum;
pub mod events;
pub mod ledger_state;
pub mod messages;
pub mod state_machine;
pub mod state_store;

pub mod actions;
mod actor_state;
mod ledger;
mod save_state;
mod secret;

pub use self::{
    actor_state::ActorState,
    ledger::Ledger,
    ledger_state::{HtlcState, LedgerState},
    save_state::SaveState,
    secret::{FromErr, Secret, SecretHash},
};

pub use self::messages::{Accept, Decline, Request};

use crate::{seed::Seed, swap_protocols::asset::Asset};
use ::bitcoin::secp256k1::SecretKey;

/// Swap request response as received from peer node acting as Bob.
pub type Response<AL, BL> = Result<Accept<AL, BL>, Decline>;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("btsieve")]
    Btsieve,
    #[error("timer error")]
    TimerError,
    #[error("incorrect funding")]
    IncorrectFunding,
    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SwapCommunication<AL: Ledger, BL: Ledger, AA: Asset, BA: Asset> {
    Proposed {
        request: Request<AL, BL, AA, BA>,
    },
    Accepted {
        request: Request<AL, BL, AA, BA>,
        response: Accept<AL, BL>,
    },
    Declined {
        request: Request<AL, BL, AA, BA>,
        response: Decline,
    },
}

/// Both Alice and Bob use `DeriveIdentities` together with:
/// - the current cnd seed when requesting and accepting/declining a swap
/// - the seed in the state store for an ongoing swap
pub trait DeriveIdentities: Send + Sync + 'static {
    fn derive_redeem_identity(&self) -> SecretKey;
    fn derive_refund_identity(&self) -> SecretKey;
}

impl DeriveIdentities for Seed {
    fn derive_redeem_identity(&self) -> SecretKey {
        SecretKey::from_slice(self.sha256_with_seed(&[b"REDEEM"]).as_ref())
            .expect("The probability of this happening is < 1 in 2^120")
    }

    fn derive_refund_identity(&self) -> SecretKey {
        SecretKey::from_slice(self.sha256_with_seed(&[b"REFUND"]).as_ref())
            .expect("The probability of this happening is < 1 in 2^120")
    }
}

pub trait DeriveSecret: Send + Sync + 'static {
    fn derive_secret(&self) -> Secret;
}

impl DeriveSecret for Seed {
    fn derive_secret(&self) -> Secret {
        self.sha256_with_seed(&[b"SECRET"]).into()
    }
}
