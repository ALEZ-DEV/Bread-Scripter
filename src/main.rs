use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
};

use axum::{body::Body, http::Request, routing::get, Router};
use tokio::fs;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod serve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let current_path = env::current_dir()?;
    let config_file_path = current_path.join(config::CONFIG_FILE_NAME);

    let config: config::Config = match fs::read_to_string(config_file_path).await {
        Ok(content) => match toml::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(
                    "Failed to read config file, using default one, please check your config : {}",
                    e
                );
                config::Config::default()
            }
        },
        Err(e) => {
            tracing::warn!(
                "Seems that your config file is missing, creating default configuration for you...."
            );
            let config = config::Config::default();
            let config_path = current_path.join(config::CONFIG_FILE_NAME);
            let new_config_file_content = toml::to_string(&config).unwrap();

            fs::write(config_path, new_config_file_content.as_bytes()).await?;

            config
        }
    };

    let ip = config.serve_at.ip.parse::<Ipv4Addr>().unwrap();
    let port = config.serve_at.port;

    let addr = SocketAddr::from((ip, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let dir_to_serve = current_path.clone();
    let router = Router::new().fallback_service(get(move |req: Request<Body>| {
        serve::serve_dir(req, dir_to_serve, config.mislead)
    }));

    tracing::info!("listening on \t-> {}", listener.local_addr().unwrap());
    tracing::info!("serving dir \t-> {}", current_path.to_str().unwrap());
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
