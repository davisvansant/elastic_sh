use url::Url;
use reqwest::{Request, Method};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/cat-nodes.html

#[allow(dead_code)]
fn get(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/_cat/nodes").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn get_nodes() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = get(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_cat/nodes");
        assert_eq!(request_url.path(), "/_cat/nodes");
    }
}
