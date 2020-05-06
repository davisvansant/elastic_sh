use url::Url;
use reqwest::{Request, Method};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html

#[allow(dead_code)]
fn get(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/_search").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn post(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/_search").unwrap();

    reqwest::Request::new(method, url)
}


#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_search_url() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = get(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_search");
        assert_eq!(request_url.path(), "/_search");
    }

    #[tokio::test]
    async fn search_get() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = get(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn search_post() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = post(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
