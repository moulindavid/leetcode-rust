use std::collections::HashMap;

struct TimeMap {
    key_store: HashMap<String, Vec<(String, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            key_store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.key_store
            .entry(key)
            .or_default()
            .push((value, timestamp));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let mut result = String::new();
        if let Some(list_of_timestamps) = self.key_store.get(&key) {
            let mut left = 0;
            let mut right = list_of_timestamps.len() as i32 - 1;

            while left <= right {
                let middle = (left + right) / 2;
                let middle_value = list_of_timestamps[middle as usize].clone();
                if middle_value.1 <= timestamp {
                    result = middle_value.0;
                    left = middle + 1;
                } else {
                    right = middle - 1;
                }
            }
        }
        return result;
    }
}

fn main() {}
