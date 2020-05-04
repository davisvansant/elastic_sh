use url::Url;
use reqwest::{Request, Method};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-templates.html
// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-delete-template.html
// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-get-template.html
// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-template-exists.html

#[allow(dead_code)]
fn put(base_url: Url) -> Request {
    let method = Method::PUT;
    let url = base_url.join("/_template/index_template").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn delete(base_url: Url) -> Request {
    let method = Method::DELETE;
    let url = base_url.join("/_template/index_template").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn get(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/_template").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn head(base_url: Url) -> Request {
    let method = Method::HEAD;
    let url = base_url.join("/_template/*").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_index_templete_url() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = put(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_template/index_template");
        assert_eq!(request_url.path(), "/_template/index_template");
    }

    #[tokio::test]
    async fn index_template_put() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = put(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn index_template_delete() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = delete(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn index_template_get() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = get(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn index_template_head() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = head(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
