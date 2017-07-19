#[macro_use]
extern crate clap;
use clap::App;

extern crate redis;
use redis::Commands;

use std::{thread, time};


struct Redis;

impl Redis {
    pub fn new(host: &str) -> Result<redis::Connection, redis::RedisError> {
        let client = try!(redis::Client::open(host));
        let con = client.get_connection();
        match con {
            Ok(con) => Ok(con),
            Err(e) => {
                panic!("Error connecting to redis server: {}. {}", host, e);
            },
        }
    }
}


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


    let con = Redis::new(redis_host).unwrap();
    let num_secs = 2;
    let max_bulk_size = 8000;
    let mut bulk = Vec::with_capacity(max_bulk_size);

    loop {
        //let now = std::time::SystemTime::now();
        //println!("{:?}", now.duration_since(last_push));
        let key: bool = con.exists(redis_key).unwrap();

        if key == true { //FIXME: put in a timer so we can push evey x seconds
            let line: redis::Value = con.rpop(redis_key).unwrap();
            let line_value: redis::Value = redis::from_redis_value(&line).unwrap();

            if bulk.len() >= max_bulk_size {
                println!("Push bulk...");
                bulk.truncate(0);

            } else {
                let last_push = std::time::SystemTime::now();
                bulk.push(line_value);
            }
        } else {
            println!("Cannot find key {}, sleeping for {} seconds", redis_key, num_secs);
            thread::sleep(time::Duration::from_millis(1000 * num_secs));
        }
    }
}
