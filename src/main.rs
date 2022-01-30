use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use reqwest;


#[allow(unused_must_use)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
		let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .route("/{url:.*}", web::get().to(cors_proxy_get))
    })
		.bind(("127.0.0.1", get_port()))?
		.run()
		.await;

    Ok(())
}

async fn cors_proxy_get(url: web::Path<String>) -> String {
	// TODO: make it better 
	println!("Requesting url: {}", &url);
    reqwest::get(url.as_str())
        .await
        .expect("mslolg")
        .text()
        .await
        .expect("msg")
}

fn get_port() -> u16 {
	std::env::var("PORT")
		.unwrap_or("8080".to_string())
		.parse()
		.unwrap()
}
