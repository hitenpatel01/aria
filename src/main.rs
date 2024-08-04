//External References
use std::{thread, time::Duration};

//Module Declarations
mod args;
mod settings;

//Internal References
use settings::SETTINGS;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::info!("Running with following arguments:\n{:#?}", args::ARGS);

    let market_data_cached = SETTINGS.get_bool("market_data.cached").unwrap();
    let market_data_cache_location = SETTINGS.get_string("market_data.cache.location").unwrap();
    log::info!("Market Data: Cached = {market_data_cached}, Location =  {market_data_cache_location}");

    loop {
        println!("Hello there!");
        thread::sleep(Duration::from_secs(1));
    }
}
