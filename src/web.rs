use crate::{
    routes::app_routes,
    utils::app_setup::{add_layers, app_setup},
};

pub async fn start_server() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let (config, app_state) = app_setup();

    let routes = app_routes();

    let app = add_layers(routes, app_state);

    let addr = format!("0.0.0.0:{}", config.server_port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
