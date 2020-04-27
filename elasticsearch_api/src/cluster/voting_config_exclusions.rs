use url::Url;
use reqwest::{Request, Method};

#[allow(dead_code)]
fn post(base_url: Url) -> Request {
    let method = Method::POST;
    let url = base_url.join("/_cluster/voting_config_exclusions").unwrap();

    reqwest::Request::new(method, url)
}

#[allow(dead_code)]
fn delete(base_url: Url) -> Request {
    let method = Method::DELETE;
    let url = base_url.join("/_cluster/voting_config_exclusions").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_voting_config_exclusions_url() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = post(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_cluster/voting_config_exclusions");
        assert_eq!(request_url.path(), "/_cluster/voting_config_exclusions");
    }

    #[tokio::test]
    async fn voting_config_exclusions_post() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = post(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn voting_config_exclusions_delete() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = delete(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
