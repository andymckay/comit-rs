use rand::OsRng;
use rocket;
use routes;
use std::sync::Mutex;
use types::{ExchangeApiUrl, Offers};

pub fn create_rocket_instance(exchange_api_url: ExchangeApiUrl, offers: Offers) -> rocket::Rocket {
    // TODO: allow caller to choose randomness source
    let rng = OsRng::new().expect("Failed to get randomness from OS");
    rocket::ignite()
        .mount("/", routes![routes::eth_btc::post])
        .manage(exchange_api_url)
        .manage(offers)
        .manage(Mutex::new(rng))
}