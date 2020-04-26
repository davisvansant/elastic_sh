use url::Url;
use reqwest::{Request, Method};
use serde::{Deserialize, Serialize};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/cluster-pending.html

#[derive(Serialize, Deserialize)]
struct Response {
    tasks: Vec<String>,
    insert_order: u8,
    priority: String,
    source: String,
    time_in_queue_millis: u8,
    time_in_queue: String,
}

#[allow(dead_code)]
fn get(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/_cluster/pending_tasks").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_pending_tasks() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = get(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_cluster/pending_tasks");
        assert_eq!(request_url.path(), "/_cluster/pending_tasks");
    }

    #[tokio::test]
    async fn pending_tasks_response() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = get(base);
        let response = client.execute(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
}
