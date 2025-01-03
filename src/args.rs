use clap::Parser;
use once_cell::sync::Lazy;
use std::fmt;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Environment {
    Sandbox,
    Production
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let environmment_name = match self {
            Environment::Sandbox => "Sandbox",
            Environment::Production => "Production",
        };
        write!(f, "{}", environmment_name)
    }
}

#[derive(Parser,Debug)]
pub struct Args {
    #[arg(short, long, value_enum)]
    pub env: Environment
}

pub static ARGS: Lazy<Args> = Lazy::new(|| Args::parse());