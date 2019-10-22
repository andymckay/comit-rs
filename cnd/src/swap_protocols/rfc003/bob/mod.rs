pub mod actions;
mod communication_events;
mod insert_state;
mod spawner;

pub use self::{communication_events::*, insert_state::*, spawner::*};

use crate::swap_protocols::{
    asset::Asset,
    rfc003::{
        self,
        ledger::Ledger,
        ledger_state::LedgerState,
        messages::{AcceptResponseBody, DeclineResponseBody, Request},
        secret_source::SecretSource,
        ActorState, Secret,
    },
};
use derivative::Derivative;
use futures::sync::oneshot;
use std::sync::{Arc, Mutex};

#[allow(type_alias_bounds)]
pub type ResponseSender<AL: Ledger, BL: Ledger> =
    Arc<Mutex<Option<oneshot::Sender<Result<AcceptResponseBody<AL, BL>, DeclineResponseBody>>>>>;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct State<AL: Ledger, BL: Ledger, AA: Asset, BA: Asset> {
    pub swap_communication: SwapCommunication<AL, BL, AA, BA>,
    pub alpha_ledger_state: LedgerState<AL>,
    pub beta_ledger_state: LedgerState<BL>,
    #[derivative(Debug = "ignore")]
    pub secret_source: Arc<dyn SecretSource>,
    pub secret: Option<Secret>,
    pub error: Option<rfc003::Error>,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub enum SwapCommunication<AL: Ledger, BL: Ledger, AA: Asset, BA: Asset> {
    Proposed {
        request: Request<AL, BL, AA, BA>,
    },
    Accepted {
        request: Request<AL, BL, AA, BA>,
        response: AcceptResponseBody<AL, BL>,
    },
    Declined {
        request: Request<AL, BL, AA, BA>,
        response: DeclineResponseBody,
    },
}

impl<AL: Ledger, BL: Ledger, AA: Asset, BA: Asset> State<AL, BL, AA, BA> {
    pub fn accepted(
        request: Request<AL, BL, AA, BA>,
        response: AcceptResponseBody<AL, BL>,
        secret_source: impl SecretSource,
    ) -> Self {
        Self {
            swap_communication: SwapCommunication::Accepted { request, response },
            alpha_ledger_state: LedgerState::NotDeployed,
            beta_ledger_state: LedgerState::NotDeployed,
            secret_source: Arc::new(secret_source),
            secret: None,
            error: None,
        }
    }

    pub fn declined(
        request: Request<AL, BL, AA, BA>,
        response: DeclineResponseBody,
        secret_source: impl SecretSource,
    ) -> Self {
        Self {
            swap_communication: SwapCommunication::Declined { request, response },
            alpha_ledger_state: LedgerState::NotDeployed,
            beta_ledger_state: LedgerState::NotDeployed,
            secret_source: Arc::new(secret_source),
            secret: None,
            error: None,
        }
    }

    pub fn request(&self) -> Request<AL, BL, AA, BA> {
        match &self.swap_communication {
            SwapCommunication::Accepted { request, .. }
            | SwapCommunication::Proposed { request, .. }
            | SwapCommunication::Declined { request, .. } => request.clone(),
        }
    }
}

impl<AL: Ledger, BL: Ledger, AA: Asset, BA: Asset> ActorState for State<AL, BL, AA, BA> {
    type AL = AL;
    type BL = BL;
    type AA = AA;
    type BA = BA;

    fn set_secret(&mut self, secret: Secret) {
        self.secret = Some(secret)
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
