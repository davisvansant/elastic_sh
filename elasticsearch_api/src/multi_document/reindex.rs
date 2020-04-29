use url::Url;
use reqwest::{Request, Method};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-reindex.html

#[allow(dead_code)]
fn post(base_url: Url) -> Request {
    let method = Method::POST;
    let url = base_url.join("/_reindex").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_url() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = post(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_reindex");
        assert_eq!(request_url.path(), "/_reindex");
    }

    #[tokio::test]
    async fn response() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = post(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
