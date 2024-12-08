use indexmap::IndexMap;

pub struct CacheData {
    pub map: IndexMap<String, String>,
    pub capacity: usize,
}

pub struct CacheDataReq {
    pub key: String,
    pub value: String,
}