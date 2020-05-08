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
                        SubCommand::with_name("aliases").about("Returns information about currently configured aliases to indices"),
                        SubCommand::with_name("allocation").about("Provides a snapshot of the number of shards allocated to each data node and their disk space"),
                        SubCommand::with_name("count").about("Provides quick access to a document count of individual indices or all indices in a cluster"),
                        SubCommand::with_name("fielddata").about("Returns the amount of heap memory currently used by fielddata on every data node in the cluster"),
                        SubCommand::with_name("health").about("Returns the health status of a cluster, similar to the cluster health API"),
                        SubCommand::with_name("indicies").about("Returns high-level information about indices in a cluster"),
                        SubCommand::with_name("master").about("Returns information about the master node, including the ID, bound IP address, and name"),
                        SubCommand::with_name("nodeattrs").about("Returns information about custom node attributes"),
                        SubCommand::with_name("nodes").about("Returns information about a cluster’s nodes"),
                        SubCommand::with_name("pending-tasks").about("Returns cluster-level changes that have not yet been executed"),
                        SubCommand::with_name("plugins").about("Returns a list of plugins running on each node of a cluster"),
                        SubCommand::with_name("recovery").about("Returns information about ongoing and completed shard recoveries"),
                        SubCommand::with_name("repositories").about("Returns the snapshot repositories for a cluster"),
                        SubCommand::with_name("segments").about("Returns low-level information about the Lucene segments in index shards"),
                        SubCommand::with_name("shards").about("The shards command is the detailed view of what nodes contain which shards"),
                        SubCommand::with_name("snapshots").about("Returns information about the snapshots stored in one or more repositories"),
                        SubCommand::with_name("task-management").about("Returns information about tasks currently executing in the cluster"),
                        SubCommand::with_name("templates").about("Returns information about index templates in a cluster"),
                        SubCommand::with_name("thread_pool").about("Returns thread pool statistics for each node in a cluster")
                    ]),
                SubCommand::with_name("cluster").about("Elasticsearch cluster api")
                    .display_order(2),
                SubCommand::with_name("index").about("Elasticsearch index api")
                    .display_order(3),
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
