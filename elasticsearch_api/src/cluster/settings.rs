use url::Url;
use reqwest::{Request, Method};
// use serde::{Deserialize, Serialize};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/cluster-update-settings.html

// #[derive(Serialize, Deserialize)]
// struct Response {
//     tasks: Vec<String>,
//     insert_order: u8,
//     priority: String,
//     source: String,
//     time_in_queue_millis: u8,
//     time_in_queue: String,
// }

#[allow(dead_code)]
fn put(base_url: Url) -> Request {
    let method = Method::PUT;
    let url = base_url.join("/_cluster/settings").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn put_settings() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = put(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_cluster/settings");
        assert_eq!(request_url.path(), "/_cluster/settings");
    }

    #[tokio::test]
    async fn response_settings() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = put(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
