use std::sync::Arc;
use axum::{
    middleware, routing::{get, Router}, Extension
};
use tower_http::trace::TraceLayer;

use crate::{handler::{file::file_handle, file_query::get_file_list_handler, user::user_handler}, middleware::auth, AppState};


pub fn create_router(app_state:Arc<AppState>) -> Router {
    let api_route=Router::new()
    .nest("/users", user_handler().layer(middleware::from_fn(auth)))
    .nest("/file", file_handle().layer(middleware::from_fn(auth)))
    .nest("/list", get_file_list_handler().layer(middleware::from_fn(auth)))
    .layer((TraceLayer::new_for_http()))
    .layer(Extension(app_state));
    // /api/users/register this example route how this work
Router::new().nest("/api",api_route)
}