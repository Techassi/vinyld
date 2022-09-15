use std::net;

use axum::{extract, routing, Router};

use crate::{config::Config, server::error::ServerError, store::Store};

mod error;
mod routes;

pub struct Server {
    is_running: bool,
    config: Config,
}

impl Server {
    pub fn new(cfg: Config) -> Self {
        Self {
            is_running: false,
            config: cfg,
        }
    }

    #[tokio::main]
    pub async fn run(&mut self) -> Result<(), ServerError> {
        if self.is_running {
            return Err(ServerError::new("Server already running"));
        }
        self.is_running = true;

        let store = match Store::new(self.config.store.clone()).await {
            Ok(store) => store,
            Err(err) => return Err(ServerError::new(format!("Store creation failed: {}", err))),
        };

        match store.migrate().await {
            Ok(_) => {}
            Err(err) => {
                return Err(ServerError::new(format!(
                    "Failed to migrate one ore more tables: {}",
                    err
                )))
            }
        };

        let vinyl_router = Router::new()
            .route("/", routing::post(routes::create_media_entry))
            .route("/", routing::get(routes::get_media_entries))
            .route("/:id", routing::get(routes::get_media_entry))
            .route("/:id", routing::post(routes::update_media_entry))
            .route("/:id", routing::delete(routes::delete_media_entry));

        let router = Router::new()
            .nest("/api/media", vinyl_router)
            .layer(extract::Extension(store));

        let address: net::SocketAddr = match self.config.server.address.parse() {
            Ok(addr) => addr,
            Err(err) => {
                return Err(ServerError::new(format!(
                    "Failed to parse socket address: {}",
                    err
                )))
            }
        };

        axum::Server::bind(&address)
            .serve(router.into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}
