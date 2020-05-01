use url::Url;
use reqwest::{Request, Method};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html
// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-delete-index.html

#[allow(dead_code)]
fn put(base_url: Url) -> Request {
    let method = Method::PUT;
    let url = base_url.join("/test_index").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn delete(base_url: Url) -> Request {
    let method = Method::DELETE;
    let url = base_url.join("/test_index").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_index_management_url() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = put(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/test_index");
        assert_eq!(request_url.path(), "/test_index");
    }

    #[tokio::test]
    async fn index_management_create() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = put(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn index_management_delete() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = delete(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
