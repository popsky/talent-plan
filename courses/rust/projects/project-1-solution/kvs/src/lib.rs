use std::collections::HashMap;

/// A key-value store that support get,set,rm operations.
///
/// # Examples
///
/// ```
/// use kvs::KvStore;
///
/// let mut kv = KvStore::new();
/// kv.set("foo".to_owned(),"bar".to_owned());
/// assert_eq!(kv.get("foo".to_owned()).unwrap(), "bar".to_owned());
/// kv.remove("foo".to_owned());
/// assert_eq!(kv.get("foo".to_owned()), None);
pub struct KvStore {
    m: HashMap<String, String>,
}
impl KvStore {
    pub fn new() -> KvStore {
        KvStore { m: HashMap::new() }
    }
    pub fn get(&self, key: String) -> Option<String> {
        let v = self.m.get(&key);
        if let Some(v) = v {
            return Some(String::from(v));
        }
        None
    }
    pub fn set(&mut self, key: String, value: String) {
        self.m.insert(key, value);
    }
    pub fn remove(&mut self, key: String) {
        self.m.remove(&key);
    }
}
