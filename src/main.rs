// #![allow(unused)] // For beginning only.

pub use self::error::{Error, Result};

use crate::model::ModelController;

use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time()
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// Initialize ModelController.
	let mc = ModelController::new().await?;

	let routes_apis = web::routes_tickets::routes(mc.clone())
		.route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

	let routes_all = Router::new()
		.merge(web::routes_login::routes())
		.nest("/api", routes_apis)
		.layer(middleware::map_response(
			web::mw_res_map::main_response_mapper,
		))
		.layer(middleware::from_fn_with_state(
			mc.clone(),
			web::mw_auth::mw_ctx_resolver,
		))
		.layer(CookieManagerLayer::new())
		.fallback_service(web::routes_static::routes_static());

	// region:    --- Start Server
	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	info!("LISTENING on {addr}\n");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}
