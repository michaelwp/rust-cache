use super::data::{CacheData, CacheDataReq};
use indexmap::IndexMap;

pub trait CacheInterface {
    fn set_cache_value(&mut self, req: CacheDataReq);
    fn get_cache_value(&mut self, key: String) -> String;
    fn show_cache_data(&self);
}

impl CacheInterface for CacheData {
    fn set_cache_value(&mut self, req: CacheDataReq) {
        if self.map.len() >= self.capacity {
            let shift_remove_req: ShiftRemoveReq = ShiftRemoveReq {
                key: String::from(req.key),
                value: String::from(req.value),
                element_index: 0,
            };

            <ShiftRemoveReq as PrivateCacheInterface>::shift_remove_cache_map(
                &mut self.map,
                shift_remove_req,
            );
        } else {
            self.map.insert(req.key, req.value);
        }
    }

    fn get_cache_value(&mut self, key: String) -> String {
        let cache_value: String = self.map.get(&key).unwrap().clone();

        let shift_remove_req: ShiftRemoveReq = ShiftRemoveReq {
            key: String::from(&key),
            value: String::from(&cache_value),
            element_index: self.map.get_index_of(&key).unwrap(),
        };

        <ShiftRemoveReq as PrivateCacheInterface>::shift_remove_cache_map(
            &mut self.map,
            shift_remove_req,
        );

        return cache_value;
    }

    fn show_cache_data(&self) {
        self.map.iter().for_each(|(k, v)| {
            println!("{}: {}", k, v);
        });

        println!("==========");
    }
}

struct ShiftRemoveReq {
    key: String,
    value: String,
    element_index: usize,
}

trait PrivateCacheInterface {
    fn shift_remove_cache_map(cache_map: &mut IndexMap<String, String>, req: ShiftRemoveReq);
}

impl PrivateCacheInterface for ShiftRemoveReq {
    fn shift_remove_cache_map(cache_map: &mut IndexMap<String, String>, req: ShiftRemoveReq) {
        cache_map.shift_remove_index(req.element_index);
        cache_map.insert(req.key, req.value);
    }
}
