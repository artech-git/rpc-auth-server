extern crate once_cell;

use config::Config;
use once_cell::sync::{Lazy, OnceCell};
use std::collections::HashMap;

pub const FILE_PATH: &str = "./settings.toml";

pub static KEY_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let settings = match Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name(FILE_PATH))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
    {
        Ok(v) => {
            tracing::log::info!("settings.toml read");
            v
        }
        Err(e) => {
            tracing::log::error!("cannot open the file: {}", e);
            panic!();
        }
    };

    let hm = match settings.try_deserialize::<HashMap<String, String>>() {
        Ok(v) => {
            tracing::log::info!("deserialization succesfull");
            v
        }
        Err(e) => {
            tracing::log::error!("error deserialization of values: {}", e);
            panic!();
        }
    };

    return hm;
});
