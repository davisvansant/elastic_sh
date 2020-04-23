use url::Url;
use reqwest::{Request, Method};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/cat-allocation.html

// struct PathParameters {
//     node_id: Option<String>,
//     // node_id: Vec<String>,
// }
//
// struct QueryParameters {
//     bytes: Option<String>,
//     format: Option<String>,
//     local: Option<bool>,
//     master_timeout: Option<Duration>,
//     h: Option<String>,
//     help: Option<bool>,
//     s: Option<String>,
//     v: Option<String>,
// }

#[allow(dead_code)]
fn get(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/_cat/allocation").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn get_allocation() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = get(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_cat/allocation");
        assert_eq!(request_url.path(), "/_cat/allocation");
    }
}
