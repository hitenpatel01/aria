use config::Config;
use clap::Parser;
use std::{thread, time::Duration};

mod args;
use args::Args;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let args = Args::parse();

    log::info!("Running with following arguments:\n{:#?}", args);

    let env_config_file = format!("settings/settings_{0}", args.env.to_string().to_lowercase());
    
    let settings = Config::builder()
        .add_source(config::File::with_name("settings/settings"))
        .add_source(config::File::with_name(&env_config_file))
        .add_source(config::Environment::with_prefix("MM"))
        .build()
        .unwrap();

    let market_data_cached = settings.get_bool("market_data.cached").unwrap();
    let market_data_cache_location = settings.get_string("market_data.cache.location").unwrap();
    log::info!("Market Data: Cached = {market_data_cached}, Location =  {market_data_cache_location}");

    loop {
        println!("Hello there!");
        thread::sleep(Duration::from_secs(5));
    }
}
