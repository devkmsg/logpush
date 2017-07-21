extern crate redis;
use self::redis::Commands;

pub struct Redis {
    pub connection: redis::Connection,
    key: String
}

impl Redis {
    pub fn new(host: String, key: String) -> Redis {
        Redis {
            connection: Redis::connect(host).unwrap(),
            key: key
        }
    }

    fn connect(host: String) -> Result<redis::Connection, redis::RedisError> {
        let host: &str = &host;
        let client = try!(self::redis::Client::open(host));
        let con = client.get_connection();
        match con {
            Ok(con) => Ok(con),
            Err(e) => {
                panic!("Error connecting to redis server: {}. {}", host, e);
            },
        }
    }

    pub fn pop(&self) -> redis::Value {
        let line: self::redis::Value = self.connection.rpop(&self.key).unwrap();
        self::redis::from_redis_value(&line).unwrap()
    }

    pub fn key_exists(&self) -> bool {
        self.connection.exists(self.key.clone()).unwrap()
    }
}
