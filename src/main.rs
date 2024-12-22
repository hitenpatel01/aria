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

    let data_cached = SETTINGS.get_bool("data.cached").unwrap();
    let data_cache_location = SETTINGS.get_string("data.cache.location").unwrap();
    log::info!("Data: Cached = {data_cached}, Location =  {data_cache_location}");

    loop {
        println!("Hello there!");
        thread::sleep(Duration::from_secs(1));
    }
}
