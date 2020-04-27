use url::Url;
use reqwest::{Request, Method};
use serde::{Deserialize, Serialize};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/cluster-state.html

#[derive(Serialize, Deserialize)]
enum Metrics {
    All(String),
    Blocks(String),
    MasterNode(String),
    Metadata(String),
    Nodes(String),
    RoutingNodes(String),
    RoutingTable(String),
    Version(String),
}

// #[derive(Serialize, Deserialize)]
// struct Index {
//
// }

#[allow(dead_code)]
fn get(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/_cluster/state").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_state() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = get(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_cluster/state");
        assert_eq!(request_url.path(), "/_cluster/state");
    }

    #[tokio::test]
    async fn state_response() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = get(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
