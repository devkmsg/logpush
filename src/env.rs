
use clap::App;

pub struct ENV {
    pub redis_host: String,
    pub redis_key: String,
    pub elasticsearch_host: String,
    pub index_pattern: String,
    pub max_bulk_size: usize,
    pub max_bulk_interval_secs: usize

}

impl ENV {
    pub fn new() -> ENV {
        // The YAML file is found relative to the current file, similar to how modules are found
        let yaml = load_yaml!("../cli.yml");
        let matches = App::from_yaml(yaml).get_matches();
        // Convert Strings to u8
        let max_bulk_size_str = matches.value_of("max_bulk_size").unwrap_or("default.conf").to_string();
        let max_bulk_size = max_bulk_size_str.parse::<usize>().unwrap();
        let max_bulk_interval_secs_str = matches.value_of("max_bulk_interval_secs").unwrap_or("default.conf").to_string();
        let max_bulk_interval_secs = max_bulk_interval_secs_str.parse::<usize>().unwrap();

        ENV {
            redis_host: matches.value_of("redis_host").unwrap_or("default.conf").to_string(),
            redis_key: matches.value_of("redis_key").unwrap_or("default.conf").to_string(),
            elasticsearch_host: matches.value_of("elasticsearch_host").unwrap_or("default.conf").to_string(),
            max_bulk_size: max_bulk_size,
            max_bulk_interval_secs: max_bulk_interval_secs,
            index_pattern: matches.value_of("index_pattern").unwrap_or("default.conf").to_string()
        }
    }

    pub fn print(&self) {
        println!("Value for redis_host: {}", self.redis_host);
        println!("Value for redis_key: {}", self.redis_key);
        println!("Value for elasticsearch_host: {}", self.elasticsearch_host);
        println!("Value for max_bulk_size: {}", self.max_bulk_size);
        println!("Value for max_bulk_interval_secs: {}", self.max_bulk_interval_secs);
        println!("Value for index_pattern: {}", self.index_pattern);
    }
}
