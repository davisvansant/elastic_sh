use url::Url;
use reqwest::{Request, Method};
use serde::{Deserialize, Serialize};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/cluster-health.html

#[derive(Serialize, Deserialize)]
struct Response {
    cluster_name: String,
    status: String,
    timed_out: bool,
    number_of_nodes: u8,
    number_of_data_nodes: u8,
    active_primary_shards: u8,
    active_shards: u8,
    relocating_shards: u8,
    initializing_shards: u8,
    unassigned_shards: u8,
    delayed_unassigned_shards: u8,
    number_of_pending_tasks: u8,
    number_of_in_flight_fetch: u8,
    task_max_waiting_in_queue_millis: u8,
    active_shards_percent_as_number: f32,
}

#[allow(dead_code)]
fn get(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/_cluster/health").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_health() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = get(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_cluster/health");
        assert_eq!(request_url.path(), "/_cluster/health");
    }

    #[tokio::test]
    async fn health_response() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = get(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
