use clap::{App, SubCommand};

fn main() {
    App::new("elastic")
        .version("0.1.0")
        .about("| >_ a shell and cli for elastic.co products")
        .subcommand(SubCommand::with_name("es").about("Elasticsearch api")
            .subcommands( vec![
                SubCommand::with_name("cat").about("Elasticsearch cat api")
                    .display_order(1),
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
