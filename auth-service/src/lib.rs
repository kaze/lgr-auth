use std::error::Error;

use axum::Router;
use axum::routing::{post};
use axum::serve::Serve;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::app_state::AppState;

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
                .fallback_service(ServeDir::new("assets"))
                .route("/signup", post(routes::signup))
                .route("/login", post(routes::login))
                .route("/logout", post(routes::logout))
                .route("/verify-2fa", post(routes::verify_2fa))
                .route("/verify-token", post(routes::verify_token))
                .with_state(app_state);

        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        let app = Application { server, address };
        Ok(app)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
