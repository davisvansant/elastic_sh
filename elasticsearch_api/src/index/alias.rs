use url::Url;
use reqwest::{Request, Method};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-add-alias.html
// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-delete-alias.html
// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-get-alias.html
// https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-alias-exists.html

#[allow(dead_code)]
fn put(base_url: Url) -> Request {
    let method = Method::PUT;
    let url = base_url.join("/index/_alias/alias").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn post(base_url: Url) -> Request {
    let method = Method::POST;
    let url = base_url.join("/index/_alias/alias").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn delete(base_url: Url) -> Request {
    let method = Method::DELETE;
    let url = base_url.join("/index/_alias/alias").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn head(base_url: Url) -> Request {
    let method = Method::HEAD;
    let url = base_url.join("/index/_alias/alias").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn get(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/index/_alias/alias").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_index_alias_url() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = put(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/index/_alias/alias");
        assert_eq!(request_url.path(), "/index/_alias/alias");
    }

    #[tokio::test]
    async fn index_alias_put() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = put(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn index_alias_post() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = post(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn index_alias_delete() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = delete(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn index_alias_get() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = get(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn index_alias_head() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = head(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
