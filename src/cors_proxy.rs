use std::collections::HashMap;
use std::str::FromStr;

use actix_web::{web, FromRequest, HttpRequest, Responder};
use reqwest;
use serde_json::Value;

pub struct CorsProxy;

impl CorsProxy {
    pub async fn get(request: HttpRequest) -> impl Responder {
        CorsProxy::new().request(request).await
    }

    pub async fn post(request: HttpRequest) -> impl Responder {
        CorsProxy::new().request(request).await
    }

    pub(self) fn new() -> CorsProxy {
        CorsProxy {}
    }

    async fn request(self, request: HttpRequest) -> impl Responder {
        let client = reqwest::Client::new();

        let url = self.get_url_from_request(&request);
        let headers = {
            let actix_headermap = request.headers().to_owned();
            let mut hashmap = actix_headermap_to_hashmap(&actix_headermap);

			hashmap.remove("host"); // removing host header
			hashmap.remove("referer"); // removing referer header
			hashmap.remove("origin"); // removing origin header
			hashmap.remove("accept-encoding"); // removing accept-encoding header

            reqwest_headermap_from_hashmap(hashmap.iter())
        };
		println!("\n{:?}", &headers);

        let request_builder = match request.method().as_str() {
            "GET" => client.get(&url),
            "POST" => {
                let bytes_json = web::Bytes::extract(&request)
                    .await
                    .expect("Error on extracting bytes from body");
                let string_json = String::from_utf8(bytes_json.to_vec())
                    .expect("Some wrong body encoding (it's invalid UTF-8)");
                let json: Value =
                    serde_json::from_str(string_json.as_str()).expect("Some wrong JSON");
                client.post(&url).json(&json)
            }
            _ => unimplemented!("Method not allowed"),
        };

        println!("Requesting url: {}", url);
        request_builder
			.headers(headers) // adding headers
            .send()
            .await
            .expect("Some wrong url or server or client")
            .text()
            .await
            .expect("response.text is wrong")
    }

    fn get_url_from_request(self, request: &HttpRequest) -> String {
        // Removing first slash from relative path
        let request_path = &request.path()[1..request.path().len()];
        format!("{}?{}", request_path, request.query_string())
    }
}

/// Converting actix_web headermap to hashmap
/// https://blog.thomasheartman.com/posts/building-a-request-inspector
fn actix_headermap_to_hashmap(
    headers: &actix_web::http::header::HeaderMap,
) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (header_name, header_value) in headers.iter() {
        let k = header_name.as_str();
        let v = header_value
            .to_str()
            .unwrap_or("Non-ASCII header value")
            .into();

        match map.get_mut(k) {
            None => {
                map.insert(k.into(), v);
            }
            Some(old_val) => *old_val = format!("{}, {}", old_val, v),
        }
    }

    map
}

/// Converting hashmap to reqwest headermap
/// https://github.com/seanmonstar/reqwest/issues/555#issuecomment-507566071
fn reqwest_headermap_from_hashmap<'a, I, S>(headers: I) -> reqwest::header::HeaderMap
where
    I: Iterator<Item = (S, S)> + 'a,
    S: AsRef<str> + 'a,
{
    headers
        .map(|(name, val)| {
            (
                reqwest::header::HeaderName::from_str(name.as_ref()),
                reqwest::header::HeaderValue::from_str(val.as_ref()),
            )
        })
        // We ignore the errors here. If you want to get a list of failed conversions, you can use Iterator::partition
        // to help you out here
        .filter(|(k, v)| k.is_ok() && v.is_ok())
        .map(|(k, v)| (k.unwrap(), v.unwrap()))
        .collect()
}
