extern crate r2d2_redis;

use lazy_static::lazy_static;

use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;

lazy_static! {
    static ref POOL: r2d2::Pool<RedisConnectionManager> = r2d2::Pool::builder().build(RedisConnectionManager::new("redis://localhost").unwrap()).unwrap();
}

pub async fn set_value(field: &str, value: &str) -> String {
    let mut con = POOL.get().unwrap();
    let n: String = match con.set(field, value) {
        Ok(res) => {
            res
        }, Err(_why) => {
            String::from("")
        }
    };
    n
}

pub async fn get_value(field: &str) -> String {
    let mut con = POOL.get().unwrap();
    let n: String = match con.get(field) {
        Ok(res) => {
            res
        }, Err(_why) => {
            String::from("")
        }
    };
    n
}