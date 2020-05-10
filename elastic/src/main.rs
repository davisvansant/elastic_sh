use clap::{App, SubCommand};

fn main() {
    App::new("elastic")
        .version("0.1.0")
        .about("| >_ a shell and cli for elastic.co products")
        .subcommand(SubCommand::with_name("es").about("Elasticsearch api")
            .subcommands( vec![
                SubCommand::with_name("cat").about("Elasticsearch cat api")
                    .display_order(1)
                    .subcommands( vec![
                            SubCommand::with_name("aliases")
                                .about("Returns information about currently configured aliases to indices"),
                            SubCommand::with_name("allocation")
                                .about("Provides a snapshot of the number of shards allocated to each data node and their disk space"),
                            SubCommand::with_name("count")
                                .about("Provides quick access to a document count of individual indices or all indices in a cluster"),
                            SubCommand::with_name("fielddata")
                                .about("Returns the amount of heap memory currently used by fielddata on every data node in the cluster"),
                            SubCommand::with_name("health")
                                .about("Returns the health status of a cluster, similar to the cluster health API"),
                            SubCommand::with_name("indicies")
                                .about("Returns high-level information about indices in a cluster"),
                            SubCommand::with_name("master")
                                .about("Returns information about the master node, including the ID, bound IP address, and name"),
                            SubCommand::with_name("nodeattrs")
                                .about("Returns information about custom node attributes"),
                            SubCommand::with_name("nodes")
                                .about("Returns information about a cluster’s nodes"),
                            SubCommand::with_name("pending-tasks")
                                .about("Returns cluster-level changes that have not yet been executed"),
                            SubCommand::with_name("plugins")
                                .about("Returns a list of plugins running on each node of a cluster"),
                            SubCommand::with_name("recovery")
                                .about("Returns information about ongoing and completed shard recoveries"),
                            SubCommand::with_name("repositories")
                                .about("Returns the snapshot repositories for a cluster"),
                            SubCommand::with_name("segments")
                                .about("Returns low-level information about the Lucene segments in index shards"),
                            SubCommand::with_name("shards")
                                .about("The shards command is the detailed view of what nodes contain which shards"),
                            SubCommand::with_name("snapshots")
                                .about("Returns information about the snapshots stored in one or more repositories"),
                            SubCommand::with_name("task-management")
                                .about("Returns information about tasks currently executing in the cluster"),
                            SubCommand::with_name("templates")
                                .about("Returns information about index templates in a cluster"),
                            SubCommand::with_name("thread_pool")
                                .about("Returns thread pool statistics for each node in a cluster")
                        ]),
                SubCommand::with_name("cluster").about("Elasticsearch cluster api")
                    .display_order(2)
                    .subcommands( vec![
                            SubCommand::with_name("allocation-explain")
                                .about("Provides explanations for shard allocations in the cluster"),
                            SubCommand::with_name("health")
                                .about("Returns the health status of a cluster"),
                            SubCommand::with_name("pending-tasks")
                                .about("Returns cluster-level changes that have not yet been executed"),
                            SubCommand::with_name("settings")
                                .about("Returns cluster-wide settings"),
                            SubCommand::with_name("state")
                                .about("Returns metadata about the state of the cluster"),
                            SubCommand::with_name("stats")
                                .about("Returns cluster statistics"),
                            SubCommand::with_name("voting-configuration-exclusions")
                                .about("Adds or removes master-eligible nodes from the voting configuration exclusion list"),
                        ]),
                SubCommand::with_name("index").about("Elasticsearch index api")
                    .display_order(3)
                    .subcommands( vec![
                            SubCommand::with_name("alias")
                                .about("An index alias is a logical name used to reference one or more indices")
                                .subcommands( vec![
                                        SubCommand::with_name("get")
                                            .about("Returns information about one or more index aliases"),
                                        SubCommand::with_name("delete")
                                            .about("Deletes an existing index alias"),
                                        SubCommand::with_name("add")
                                            .about("Creates or updates an index alias"),
                                        SubCommand::with_name("exists")
                                            .about("Checks if an index alias exists"),
                                        SubCommand::with_name("update")
                                            .about("Adds or removes index aliases")
                                    ]),
                            SubCommand::with_name("management")
                                .about("Index Management")
                                .subcommands( vec![
                                        SubCommand::with_name("create")
                                            .about("Creates a new index"),
                                        SubCommand::with_name("delete")
                                            .about("Deletes an existing index"),
                                        SubCommand::with_name("get")
                                            .about("Returns information about one or more indexes"),
                                        SubCommand::with_name("exists")
                                            .about("Checks if an index exists"),
                                        SubCommand::with_name("close")
                                            .about("Closes an index"),
                                        SubCommand::with_name("open")
                                            .about("Opens a closed index"),
                                        SubCommand::with_name("shrink")
                                            .about("Shrinks an existing index into a new index with fewer primary shards"),
                                        SubCommand::with_name("split")
                                            .about("Splits an existing index into a new index with more primary shards"),
                                        SubCommand::with_name("clone")
                                            .about("Clones an existing index"),
                                        SubCommand::with_name("rollover")
                                            .about("Assigns an index alias to a new index when the alias’s existing index meets a condition you provide."),
                                    ]),
                            SubCommand::with_name("mapping")
                                .about("Mapping Management")
                                .subcommands( vec![
                                        SubCommand::with_name("get")
                                            .about("Retrieves mapping definitions for indices in a cluster"),
                                        SubCommand::with_name("field")
                                            .about("Retrieves mapping definitions for one or more fields"),
                                        SubCommand::with_name("put")
                                            .about("Adds new fields to an existing index or changes the search settings of existing fields"),

                                    ]),
                            SubCommand::with_name("monitoring")
                                .about("Monitoring")
                                .subcommands( vec! [
                                        SubCommand::with_name("stats")
                                            .about("Returns statistics for an index"),
                                        SubCommand::with_name("segments")
                                            .about("Returns low-level information about the Lucene segments in index shards"),
                                        SubCommand::with_name("recovery")
                                            .about("Returns information about ongoing and completed shard recoveries"),
                                        SubCommand::with_name("shard-stores")
                                            .about("Returns store information about replica shards in one or more indices")
                                    ]),
                            SubCommand::with_name("settings")
                                .about("Index Settings")
                                .subcommands( vec! [
                                        SubCommand::with_name("update")
                                            .about("Changes an index setting in real time"),
                                        SubCommand::with_name("get")
                                            .about("Returns setting information for an index"),
                                        SubCommand::with_name("analyze")
                                            .about("Performs analysis on a text string and returns the resulting tokens")
                                    ]),
                            SubCommand::with_name("status")
                                .about("Status Management")
                                .subcommands( vec! [
                                        SubCommand::with_name("clear-cache")
                                            .about("Clears caches for one or more indices"),
                                        SubCommand::with_name("flush")
                                            .about("Flushes one or more indices"),
                                        SubCommand::with_name("force-merge")
                                            .about("Forces a merge on the shards of one or more indices"),
                                        SubCommand::with_name("refresh")
                                            .about("Refreshes one or more indices")
                                    ])
                        ]),
                SubCommand::with_name("ingest").about("Elasticsearch ingest api")
                    .display_order(4),
                SubCommand::with_name("multi-document").about("Elasticsearch multi document api")
                    .display_order(5),
                SubCommand::with_name("nodes").about("Elasticsearch nodes api")
                    .display_order(6),
                SubCommand::with_name("search").about("Elasticsearch search api")
                    .display_order(7),
                SubCommand::with_name("single-document").about("Elasticsearch single document api")
                    .display_order(8),
            ])
        )
        .get_matches();
}
