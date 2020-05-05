use url::Url;
use reqwest::{Request, Method};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/put-pipeline-api.html

#[allow(dead_code)]
fn put(base_url: Url) -> Request {
    let method = Method::PUT;
    let url = base_url.join("/_ingest/pipeline/pipeline").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_ingest_pipeline_url() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = put(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_ingest/pipeline/pipeline");
        assert_eq!(request_url.path(), "/_ingest/pipeline/pipeline");
    }

    #[tokio::test]
    async fn ingest_pipeline_create() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = put(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
