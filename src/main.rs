use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};

use cors_proxy::CorsProxy;

mod cors_proxy;

#[allow(unused_must_use)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("Server is running on port {}", get_port());
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .route("/", web::get().to(|| HttpResponse::Ok().body("Usage: GET/POST /URL")))
            .route("/{path:.+}", web::get().to(CorsProxy::get))
            .route("/{path:^http.+}", web::post().to(CorsProxy::post))
    })
    .bind(("0.0.0.0", get_port()))?
    .run()
    .await;

    Ok(())
}


fn get_port() -> u16 {
    std::env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse()
        .unwrap()
}
