use std::collections::HashMap;
use std::str::FromStr;

use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder};
use reqwest;

pub struct CorsProxy;

impl CorsProxy {
    pub async fn get(request: HttpRequest) -> HttpResponse {
        CorsProxy::new().request(request, None).await
    }

    pub async fn post(request: HttpRequest, body: String) -> HttpResponse {
        CorsProxy::new().request(request, Some(body)).await
    }

    pub(self) fn new() -> CorsProxy {
        CorsProxy {}
    }

    async fn request(self, request: HttpRequest, body: Option<String>) -> HttpResponse {
        let client = reqwest::Client::new();

        let url = {
            // Removing first slash from relative path
            let request_path = &request.path()[1..request.path().len()];
            format!("{}?{}", request_path, request.query_string())
        };

        let headers = {
            let actix_headermap = request.headers().to_owned();
            let mut hashmap = actix_headermap_to_hashmap(&actix_headermap);

            hashmap.remove("host"); // removing host header
            hashmap.remove("referer"); // removing referer header
            hashmap.remove("origin"); // removing origin header
            hashmap.remove("accept-encoding"); // removing accept-encoding header
            hashmap.remove("sec-fetch-site"); // removing sec-fetch-site header
            hashmap.remove("sec-fetch-mode"); // removing sec-fetch-mode header
            hashmap.remove("sec-fetch-dest"); // removing sec-fetch-dest header

            reqwest_headermap_from_hashmap(hashmap.iter())
        };

        let request_builder = match request.method().as_str() {
            "GET" => client.get(&url),
            "POST" => client.post(&url).body(body.unwrap()),
            _ => unimplemented!("Method not allowed"),
        };

        println!("Requesting url: {}", url);
        let response = request_builder
            .headers(headers) // adding headers
            .send()
            .await
            .expect("Some wrong url or server or client");

        // Constructing response
        let code = response.status();
        let body = response.bytes().await.expect("Can't load response body");
        HttpResponseBuilder::new(code).body(body)
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
