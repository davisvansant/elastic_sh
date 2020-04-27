use url::Url;
use reqwest::{Request, Method};
use serde::{Deserialize, Serialize};

// https://www.elastic.co/guide/en/elasticsearch/reference/current/cluster-stats.html

#[derive(Serialize, Deserialize)]
struct Response {
    underscore_nodes: UnderscoreNodes,
    cluster_name: String,
    cluster_uuid: String,
    timestamp: u8,
    status: String,
    indicies: Indicies,
    nodes: Nodes,
}

#[derive(Serialize, Deserialize)]
struct UnderscoreNodes {
    total: u8,
    successful: u8,
    failed: u8,
}

#[derive(Serialize, Deserialize)]
struct Indicies {
    count: u8,
    shards: Shards,
    docs: IndiciesDocs,
    store: IndiciesStore,
    fielddata: IndiciesFieldData,
    query_cache: IndiciesQueryCache,
    completion: IndiciesCompletion,
    segments: IndiciesSegments,
}

#[derive(Serialize, Deserialize)]
struct Shards {
    total: u8,
    primaries: u8,
    replication: f32,
    index: Index,
}

#[derive(Serialize, Deserialize)]
struct Index {
    shards: IndexShards,
    primaries: IndexPrimaries,
    replication: IndexReplication,
}

#[derive(Serialize, Deserialize)]
struct IndexShards {
    min: u8,
    max: u8,
    avg: f32,
}

#[derive(Serialize, Deserialize)]
struct IndexPrimaries {
    min: u8,
    max: u8,
    avg: f32,
}

#[derive(Serialize, Deserialize)]
struct IndexReplication {
    min: u8,
    max: f32,
    avg: f32,
}

#[derive(Serialize, Deserialize)]
struct IndiciesDocs {
    count: u8,
    deleted: u8,
}

#[derive(Serialize, Deserialize)]
struct IndiciesStore {
    size: u8,
    size_in_bytes: u8,
}

#[derive(Serialize, Deserialize)]
struct IndiciesFieldData {
    memory_size: u8,
    memory_size_in_bytes: u8,
}

#[derive(Serialize, Deserialize)]
struct IndiciesQueryCache {
    memory_size: u8,
    memory_size_in_bytes: u8,
    total_count: u8,
    hit_count: u8,
    miss_count: u8,
    cache_size: u8,
    cache_count: u8,
    evictions: u8,
}

#[derive(Serialize, Deserialize)]
struct IndiciesCompletion {
    size: u8,
    size_in_bytes: u8,
}

#[derive(Serialize, Deserialize)]
struct IndiciesSegments {
    count: u8,
    memory: u8,
    memory_in_bytes: u8,
    terms_memory_in_bytes: u8,
    stored_fields_memory: u8,
    term_vectors_memory: u8,
    term_vectors_memory_in_bytes: u8,
    norms_memory: u8,
    norms_memory_in_bytes: u8,
    points_memory: u8,
    points_memory_in_bytes: u8,
    doc_values_memory:u8,
    doc_values_memory_in_bytes: u8,
    index_writer_memory: u8,
    index_writer_memory_in_bytes: u8,
    version_map_memory: u8,
    version_map_memory_in_bytes: u8,
    fixed_bit_set: u8,
    fixed_bit_set_memory_in_bytes: u8,
    max_unsafe_auto_id_timestamp: u8,
    file_sizes: u8,
}

#[derive(Serialize, Deserialize)]
struct Nodes {
    count: NodesCount,
    versions: Vec<String>,
    os: NodesOs,
    process: NodesProcess,
    jvm: NodesJvm,
    fs: NodesFs,
    plugins: NodesPlugins,
    network_types: NodesNetworkTypes,
    discovery_types: NodesDiscoveryTypes,
    packaging_types: NodesPackagingTypes,
}

#[derive(Serialize, Deserialize)]
struct NodesCount {
    total: u8,
    coordinating_only: u8,
    role: u8,
}

#[derive(Serialize, Deserialize)]
struct NodesOs {
    available_processors: u8,
    allocated_processors: u8,
    names: NodesOsNames,
    pretty_names: NodesOsPrettyNames,
    mem: NodesOsMem,
}

#[derive(Serialize, Deserialize)]
struct NodesOsNames {
    name: String,
    count: String,
}

#[derive(Serialize, Deserialize)]
struct NodesOsPrettyNames {
    pretty_name: String,
    count: String,
}

#[derive(Serialize, Deserialize)]
struct NodesOsMem {
    total: u8,
    total_in_bytes: u8,
    free: u8,
    free_in_bytes: u8,
    used: u8,
    used_in_bytes: u8,
    free_percent: u8,
    used_percent: u8
}

#[derive(Serialize, Deserialize)]
struct NodesProcess {
    cpu: NodesProcessCpu,
    open_file_descriptors: NodesOpenFileDescriptors,
}

#[derive(Serialize, Deserialize)]
struct NodesProcessCpu {
    percent: u8
}

#[derive(Serialize, Deserialize)]
struct NodesOpenFileDescriptors {
    min: u8,
    max: u8,
    avg: u8,
}

#[derive(Serialize, Deserialize)]
struct NodesJvm {
    max_uptime: u8,
    max_uptime_millis: u8,
    versions: NodesJvmVersions,
    mem: NodesJvmMem,
    threads: u8,
}

#[derive(Serialize, Deserialize)]
struct NodesJvmVersions {
    version: String,
    vm_name: String,
    vm_version: String,
    vm_vendor: String,
    bundled_jdk: bool,
    using_budled_jdk: bool,
    count: u8,
}

#[derive(Serialize, Deserialize)]
struct NodesJvmMem {
    heap_used: u8,
    heap_used_in_bytes: u8,
    heap_max: u8,
    heap_max_in_bytes: u8,
}

#[derive(Serialize, Deserialize)]
struct NodesFs {
    total: u8,
    total_in_bytes: u8,
    free: u8,
    free_in_bytes: u8,
    available: u8,
    available_in_bytes: u8
}

#[derive(Serialize, Deserialize)]
struct NodesPlugins {
    plugin: NodesOsPluginsPlugin,
}

#[derive(Serialize, Deserialize)]
struct NodesOsPluginsPlugin {
    name: String,
    version: String,
    elasticsearch_version: String,
    java_version: String,
    description: String,
    classname: String,
    extended_plugins: String,
    has_native_controller: bool,
}

#[derive(Serialize, Deserialize)]
struct NodesNetworkTypes {
    transport_types: NodesNetworkTypesTransportTypes,
    http_types: NodesNetworkTypesHttpTypes,
}

#[derive(Serialize, Deserialize)]
struct NodesNetworkTypesTransportTypes {
    transport_types: u8
}

#[derive(Serialize, Deserialize)]
struct NodesNetworkTypesHttpTypes {
    http_type: u8,
}

#[derive(Serialize, Deserialize)]
struct NodesDiscoveryTypes {
    discovery_type: u8,
}

#[derive(Serialize, Deserialize)]
struct NodesPackagingTypes {
    flavor: String,
    packaing_type: String,
    count: u8
}

#[allow(dead_code)]
fn get(base_url: Url) -> Request {
    let method = Method::GET;
    let url = base_url.join("/_cluster/stats").unwrap();

    reqwest::Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    use reqwest::{Client, StatusCode};

    #[test]
    fn get_stats() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let request = get(base);
        let request_url = request.url();

        assert_eq!(request_url.as_str(), "http://elasticsearch:9200/_cluster/stats");
        assert_eq!(request_url.path(), "/_cluster/stats");
    }

    #[tokio::test]
    async fn stats_response() {
        let base = Url::parse("http://elasticsearch:9200").unwrap();
        let client = Client::new();
        let request = get(base);
        let response = client.execute(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
