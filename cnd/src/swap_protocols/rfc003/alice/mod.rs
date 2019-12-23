mod actions;

pub use self::actions::*;

use crate::{
    seed::Seed,
    swap_protocols::{
        asset::Asset,
        rfc003::{
            self, ledger::Ledger, ledger_state::LedgerState, messages, ActorState, Secret,
            SwapCommunication,
        },
    },
};
use derivative::Derivative;

#[derive(Clone, Derivative)]
#[derivative(Debug, PartialEq)]
pub struct State<AL: Ledger, BL: Ledger, AA: Asset, BA: Asset> {
    pub swap_communication: SwapCommunication<AL, BL, AA, BA>,
    pub alpha_ledger_state: LedgerState<AL>,
    pub beta_ledger_state: LedgerState<BL>,
    #[derivative(Debug = "ignore", PartialEq = "ignore")]
    pub secret_source: Seed, // Used to derive identities and also to generate the secret.
    pub error: Option<rfc003::Error>,
}

impl<AL: Ledger, BL: Ledger, AA: Asset, BA: Asset> State<AL, BL, AA, BA> {
    pub fn proposed(request: messages::Request<AL, BL, AA, BA>, secret_source: Seed) -> Self {
        Self {
            swap_communication: SwapCommunication::Proposed { request },
            alpha_ledger_state: LedgerState::NotDeployed,
            beta_ledger_state: LedgerState::NotDeployed,
            secret_source,
            error: None,
        }
    }

    pub fn accepted(
        request: messages::Request<AL, BL, AA, BA>,
        response: messages::Accept<AL, BL>,
        secret_source: Seed,
    ) -> Self {
        Self {
            swap_communication: SwapCommunication::Accepted { request, response },
            alpha_ledger_state: LedgerState::NotDeployed,
            beta_ledger_state: LedgerState::NotDeployed,
            secret_source,
            error: None,
        }
    }

    pub fn declined(
        request: messages::Request<AL, BL, AA, BA>,
        response: messages::Decline,
        secret_source: Seed,
    ) -> Self {
        Self {
            swap_communication: SwapCommunication::Declined { request, response },
            alpha_ledger_state: LedgerState::NotDeployed,
            beta_ledger_state: LedgerState::NotDeployed,
            secret_source,
            error: None,
        }
    }

    pub fn request(&self) -> messages::Request<AL, BL, AA, BA> {
        match &self.swap_communication {
            SwapCommunication::Accepted { request, .. }
            | SwapCommunication::Proposed { request }
            | SwapCommunication::Declined { request, .. } => request.clone(),
        }
    }
}

impl<AL: Ledger, BL: Ledger, AA: Asset, BA: Asset> ActorState for State<AL, BL, AA, BA> {
    type AL = AL;
    type BL = BL;
    type AA = AA;
    type BA = BA;

    fn set_secret(&mut self, _secret: Secret) {
        // ignored because Alice already knows the secret
    }

    fn set_error(&mut self, error: rfc003::Error) {
        self.error = Some(error)
    }

    fn alpha_ledger_mut(&mut self) -> &mut LedgerState<AL> {
        &mut self.alpha_ledger_state
    }

    fn beta_ledger_mut(&mut self) -> &mut LedgerState<BL> {
        &mut self.beta_ledger_state
    }
}
