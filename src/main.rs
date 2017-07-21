#[macro_use]
extern crate clap;

mod env;
use self::env::ENV;

mod redis;
use self::redis::Redis;

use std::time::SystemTime;


fn main() {
    // Print the env settings
    let env = ENV::new();
    env.print();


    let con = Redis::new(env.redis_host, env.redis_key);
    let mut bulk = Vec::with_capacity(env.max_bulk_size);

    let mut last_bulk = SystemTime::now();

    loop {
        let now = SystemTime::now();
        let time_diff = now.duration_since(last_bulk).unwrap().as_secs();  //FIXME: not getting updated
        println!("time diff: {:?}", time_diff);

        if con.key_exists() == true {
            let line_value = con.pop();
            bulk.push(line_value);
        }

        if bulk.len() >= env.max_bulk_size {
            println!("Push bulk...");
            bulk.truncate(0);
            last_bulk = now;
        }
    }
}
