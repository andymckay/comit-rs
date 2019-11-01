use crate::{
    libp2p_comit_ext::{FromHeader, ToHeader},
    network::{ComitNode, DialInformation},
    swap_protocols::{
        self,
        asset::Asset,
        dependencies::LedgerEventDependencies,
        rfc003::{
            self,
            bob::BobSpawner,
            create_ledger_events::CreateLedgerEvents,
            messages::{Decision, SwapDeclineReason},
            InsertState,
        },
        SwapProtocol,
    },
};
use futures::Future;
use libp2p::{Swarm, Transport};
use libp2p_comit::frame;
use serde::Deserialize;
use std::{io, sync::Mutex};
use tokio::{io::AsyncRead, prelude::AsyncWrite};

pub trait Client: Send + Sync + 'static {
    fn send_rfc003_swap_request<
        AL: swap_protocols::rfc003::Ledger,
        BL: swap_protocols::rfc003::Ledger,
        AA: Asset,
        BA: Asset,
    >(
        &self,
        peer_identity: DialInformation,
        request: swap_protocols::rfc003::messages::Request<AL, BL, AA, BA>,
    ) -> Box<dyn Future<Item = rfc003::Response<AL, BL>, Error = RequestError> + Send>
    where
        LedgerEventDependencies: CreateLedgerEvents<AL, AA> + CreateLedgerEvents<BL, BA>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum RequestError {
    /// The other node had an internal error while processing the request
    InternalError,
    /// The other node produced an invalid response
    InvalidResponse,
    /// We had to establish a new connection to make the request but it failed
    Connecting(io::ErrorKind),
    /// We were unable to send the data on the existing connection
    Connection,
}

#[derive(Debug, Deserialize)]
pub struct Reason {
    pub value: SwapDeclineReason,
}

impl<
        B: InsertState + BobSpawner,
        TTransport: Transport + Send + 'static,
        TSubstream: AsyncRead + AsyncWrite + Send + 'static,
    > Client for Mutex<Swarm<TTransport, ComitNode<TSubstream, B>>>
where
    <TTransport as Transport>::Listener: Send,
    <TTransport as Transport>::Error: Send,
{
    fn send_rfc003_swap_request<
        AL: swap_protocols::rfc003::Ledger,
        BL: swap_protocols::rfc003::Ledger,
        AA: Asset,
        BA: Asset,
    >(
        &self,
        dial_information: DialInformation,
        request: rfc003::Request<AL, BL, AA, BA>,
    ) -> Box<dyn Future<Item = rfc003::Response<AL, BL>, Error = RequestError> + Send>
    where
        LedgerEventDependencies: CreateLedgerEvents<AL, AA> + CreateLedgerEvents<BL, BA>,
    {
        let request = build_swap_request(request)
            .expect("constructing a frame::OutgoingRequest should never fail!");

        let response = {
            let mut swarm = self.lock().unwrap();
            log::debug!(
                "Making swap request to {}: {:?}",
                dial_information.clone(),
                request
            );

            swarm.send_request(dial_information.clone(), request)
        };

        let response = response.then(move |result| match result {
            Ok(mut response) => {
                let decision = response
                    .take_header("decision")
                    .map(Decision::from_header)
                    .map_or(Ok(None), |x| x.map(Some))
                    .map_err(|e| {
                        log::error!(
                            "Could not deserialize header in response {:?}: {}",
                            response,
                            e,
                        );
                        RequestError::InvalidResponse
                    })?;

                match decision {
                    Some(Decision::Accepted) => {
                        match serde_json::from_value(response.body().clone()) {
                            Ok(body) => Ok(Ok(body)),
                            Err(_e) => Err(RequestError::InvalidResponse),
                        }
                    }

                    Some(Decision::Declined) => {
                        match serde_json::from_value(response.body().clone()) {
                            Ok(body) => Ok(Err(body)),
                            Err(_e) => Err(RequestError::InvalidResponse),
                        }
                    }

                    None => Err(RequestError::InvalidResponse),
                }
            }
            Err(e) => {
                log::error!(
                    "Unable to request over connection {:?}:{:?}",
                    dial_information.clone(),
                    e
                );
                Err(RequestError::Connection)
            }
        });

        Box::new(response)
    }
}

fn build_swap_request<AL: rfc003::Ledger, BL: rfc003::Ledger, AA: Asset, BA: Asset>(
    request: rfc003::Request<AL, BL, AA, BA>,
) -> Result<frame::OutboundRequest, serde_json::Error> {
    let alpha_ledger_refund_identity = request.alpha_ledger_refund_identity;
    let beta_ledger_redeem_identity = request.beta_ledger_redeem_identity;
    let alpha_expiry = request.alpha_expiry;
    let beta_expiry = request.beta_expiry;
    let secret_hash = request.secret_hash;
    let protocol = SwapProtocol::Rfc003(request.hash_function);

    Ok(frame::OutboundRequest::new("SWAP")
        .with_header("id", request.id.to_header()?)
        .with_header("alpha_ledger", request.alpha_ledger.into().to_header()?)
        .with_header("beta_ledger", request.beta_ledger.into().to_header()?)
        .with_header("alpha_asset", request.alpha_asset.into().to_header()?)
        .with_header("beta_asset", request.beta_asset.into().to_header()?)
        .with_header("protocol", protocol.to_header()?)
        .with_body(serde_json::to_value(rfc003::RequestBody::<AL, BL> {
            alpha_ledger_refund_identity,
            beta_ledger_redeem_identity,
            alpha_expiry,
            beta_expiry,
            secret_hash,
        })?))
}
