#![warn(unused_extern_crates, missing_debug_implementations)]
#![deny(unsafe_code)]
#![feature(plugin, decl_macro)]
#![feature(tool_lints)]

#[macro_use]
extern crate debug_stub_derive;
extern crate bitcoin_rpc_client;
extern crate reqwest;
extern crate serde;
extern crate tokio;
#[macro_use]
extern crate serde_derive;
extern crate bitcoin_htlc;
extern crate bitcoin_support;
extern crate bitcoin_witness;
extern crate common_types;
extern crate ethereum_support;
extern crate ethereum_wallet;
extern crate secp256k1_support;
extern crate serde_json;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate config;
extern crate event_store;
extern crate futures;
extern crate gotham;
extern crate hex;
extern crate http_api_problem;
extern crate rand;
extern crate rustc_hex;
#[macro_use]
extern crate transport_protocol;
#[macro_use]
extern crate gotham_derive;
extern crate comit_wallet;
extern crate hyper;
extern crate mime;
#[macro_use]
extern crate failure;
extern crate url;

#[cfg(test)]
extern crate pretty_env_logger;
#[cfg(test)]
extern crate spectral;
extern crate tokio_timer;

pub mod bitcoin_fee_service;

pub mod comit_client;
pub mod comit_server;
pub mod gas_price_service;
pub mod gotham_factory;
pub mod http_api;
pub mod ledger_query_service;
pub mod settings;
pub mod swap_protocols;
pub mod swaps;
