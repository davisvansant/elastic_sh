use clap::App;

fn main() {
    App::new("elastic")
        .version("0.1.0")
        .about("Elastic")
        .get_matches();
}
