/*
    - The cache could store maximum number of data based on defined capacity.
    - The data in cache is sorted from the oldest to the newest.
    - If a new data added while the cache is full, then the oldest data will be evicted
      to maintain the maximum number of data stored.
    - The data can be called by its key.
*/

use indexmap::IndexMap;
mod cache;
use cache::data::{CacheData, CacheDataReq};
use cache::interface::CacheInterface;

fn main() {
    let mut cache_data: CacheData = CacheData {
        map: IndexMap::<String, String>::new(),
        capacity: 3,
    };

    println!("cache capacity: {}", cache_data.capacity);
    cache_data.show_cache_data();

    let cache_data_req1: CacheDataReq = CacheDataReq{
        key: String::from("1"),
        value: String::from("One"),
    };

    cache_data.set_cache_value(cache_data_req1);
    cache_data.show_cache_data();

    let cache_data_req2: CacheDataReq = CacheDataReq{
        key: String::from("2"),
        value: String::from("Two"),
    };

    cache_data.set_cache_value(cache_data_req2);
    cache_data.show_cache_data();

    let cache_data_req3: CacheDataReq = CacheDataReq{
        key: String::from("3"),
        value: String::from("Three"),
    };

    cache_data.set_cache_value(cache_data_req3);
    cache_data.show_cache_data();

    let cache_data_req4: CacheDataReq = CacheDataReq{
        key: String::from("4"),
        value: String::from("Four"),
    };

    cache_data.set_cache_value(cache_data_req4);
    cache_data.show_cache_data();

    println!("3: {}", cache_data.get_cache_value(String::from("3")));
    println!("==========");

    cache_data.show_cache_data();
}