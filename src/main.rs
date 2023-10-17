use axum::{Router, response::IntoResponse, routing::get};

#[shuttle_runtime::main]
async fn shuttle() -> shuttle_axum::ShuttleAxum {
    let app = Router::new().route("/", get(hello_world));
    Ok(app.into())
}

async fn hello_world() -> impl IntoResponse {
    "Hello world!"
}
