#[macro_use]
extern crate clap;

mod env;
use self::env::ENV;

extern crate redis;
use redis::Commands;

use std::time::SystemTime;


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
    // Print the env settings
    let env = ENV::new();
    env.print();


    let con = Redis::new(&env.redis_host).unwrap();
    let mut bulk = Vec::with_capacity(env.max_bulk_size);

    let mut last_bulk = SystemTime::now();

    loop {
        let now = SystemTime::now();
        let time_diff = now.duration_since(last_bulk).unwrap().as_secs();  //FIXME: not getting updated
        println!("time diff: {:?}", time_diff);

        let key: bool = con.exists(&env.redis_key).unwrap();

        if key == true { //FIXME: put in a timer so we can push evey x seconds
            let line: redis::Value = con.rpop(&env.redis_key).unwrap();
            let line_value: redis::Value = redis::from_redis_value(&line).unwrap();
            bulk.push(line_value);
        }

        if bulk.len() >= env.max_bulk_size {
            println!("Push bulk...");
            bulk.truncate(0);
            last_bulk = now;
        }
    }
}
