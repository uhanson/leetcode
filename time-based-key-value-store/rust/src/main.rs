use std::collections::BTreeMap;
use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, BTreeMap<i32, String>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap { map: HashMap::new() }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key)
                .and_modify(|tm| { 
                    tm.insert(timestamp, value.clone()); 
                }).or_insert({
                    let mut tm = BTreeMap::new();
                    
                    tm.insert(timestamp, value);

                    tm
                });
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        self.map.get(&key).and_then(|tm| tm.range(..=timestamp).rev().next()).map(|v| v.1.as_str()).unwrap_or("").to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

 fn main() {
    println!("Hello, world!");
}
