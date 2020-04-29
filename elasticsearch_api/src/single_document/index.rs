use url::Url;
use reqwest::{Request, Method};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html

#[allow(dead_code)]
fn put(base_url: Url) -> Request {
    let method = Method::PUT;
    let url = base_url.join("/index/_create").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn post(base_url: Url) -> Request {
    let method = Method::POST;
    let url = base_url.join("/index/_create").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_index() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = post(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/index/_create");
        assert_eq!(request_url.path(), "/index/_create");
    }

    #[tokio::test]
    async fn index_put_response() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = put(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn index_post_response() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = post(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
