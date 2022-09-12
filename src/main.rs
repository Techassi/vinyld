use std::env;

mod config;
mod server;
mod store;
mod types;

fn main() {
    let config_path = env::var("CONFIG_FILE").unwrap_or(String::new());
    let cfg;

    if !config_path.is_empty() {
        cfg = match config::Config::from_file(config_path) {
            Ok(cfg) => cfg,
            Err(_) => todo!(),
        };
    } else {
        cfg = match config::Config::from_env() {
            Ok(cfg) => cfg,
            Err(_) => todo!(),
        };
    }

    let mut srv = server::Server::new(cfg);

    match srv.run() {
        Ok(_) => {}
        Err(_) => todo!(),
    }
}
