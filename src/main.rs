#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let redis_host = matches.value_of("redis_host").unwrap_or("default.conf");
    println!("Value for redis_host: {}", redis_host);

    let redis_key = matches.value_of("redis_key").unwrap_or("default.conf");
    println!("Value for redis_key: {}", redis_key);

    let elasticsearch_host = matches.value_of("elasticsearch_host").unwrap_or("default.conf");
    println!("Value for elasticsearch_host: {}", elasticsearch_host);

    let index_pattern = matches.value_of("index_pattern").unwrap_or("default.conf");
    println!("Value for index_pattern: {}", index_pattern);
}
