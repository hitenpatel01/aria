use config::Config;
use once_cell::sync::Lazy;

pub static SETTINGS: Lazy<Config> = Lazy::new(|| {
    let env_config_file = format!("settings/settings_{0}",crate::args::ARGS.env.to_string().to_lowercase());

    Config::builder()
        .add_source(config::File::with_name("settings/settings"))
        .add_source(config::File::with_name(&env_config_file))
        .add_source(config::Environment::with_prefix("ARIA"))
        .build()
        .unwrap()
});
