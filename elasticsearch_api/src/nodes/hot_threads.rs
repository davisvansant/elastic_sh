use url::Url;
use reqwest::{Request, Method};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/cluster-nodes-hot-threads.html

#[allow(dead_code)]
fn get(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/_nodes/hot_threads").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_hot_threads() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = get(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_nodes/hot_threads");
        assert_eq!(request_url.path(), "/_nodes/hot_threads");
    }

    #[tokio::test]
    async fn hot_threads_response() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = get(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
