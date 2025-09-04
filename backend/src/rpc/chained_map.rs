use anyhow::{anyhow, Result};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct ChainedMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
{
    map: HashMap<K, VecDeque<usize>>, // key → indices into `global`
    global: VecDeque<(K, V)>,         // global storage of actual values
}

impl<K, V> ChainedMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            global: VecDeque::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let idx = self.global.len(); // index of this element
        self.global.push_back((key.clone(), value));
        self.map
            .entry(key)
            .or_default()
            .push_back(idx);
    }

    /// Remove and return the first value for a given key
    pub fn remove_first_for_key(&mut self, key: &K) -> Result<Option<V>> {
        if let Some(indices) = self.map.get_mut(key) {
            if let Some(idx) = indices.pop_front() {
                // Get the element from global
                if let Some((k, _)) = self.global.get(idx) {
                    if k != key {
                        return Err(anyhow!("Key mismatch: expected {:?}, found {:?}", key, k));
                    }
                } else {
                    return Err(anyhow!("Invalid index {} in global storage", idx));
                }

                // Find and remove from global
                if let Some(pos_in_global) = self.global.iter().position(|(k, _)| k == key) {
                    let (_, value) = self.global.remove(pos_in_global).unwrap();
                    if indices.is_empty() {
                        self.map.remove(key);
                    }
                    return Ok(Some(value));
                } else {
                    return Err(anyhow!("Key {:?} not found in global storage", key));
                }
            }
        }
        Ok(None)
    }

    /// Remove and return the first inserted key-value pair globally
    pub fn remove_first_global(&mut self) -> Option<(K, V)> {
        if let Some((key, value)) = self.global.pop_front() {
            if let Some(indices) = self.map.get_mut(&key) {
                indices.pop_front();
                if indices.is_empty() {
                    self.map.remove(&key);
                }
            }
            Some((key, value))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tokio::sync::oneshot;

    #[tokio::test]
    async fn test_insert_and_remove_by_key() -> Result<()> {
        let mut bag: ChainedMap<&str, oneshot::Sender<String>> = ChainedMap::new();

        let (tx, rx) = oneshot::channel();
        bag.insert("job1", tx);

        // Remove first for key
        let sender = bag.remove_first_for_key(&"job1")?.unwrap();
        sender.send("done!".to_string()).unwrap();

        assert_eq!(rx.await.unwrap(), "done!");
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_and_remove_global() -> Result<()> {
        let mut bag: ChainedMap<&str, oneshot::Sender<String>> = ChainedMap::new();

        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        bag.insert("job1", tx1);
        bag.insert("job2", tx2);

        // Remove first globally
        let (key, sender) = bag.remove_first_global().unwrap();
        assert_eq!(key, "job1");
        sender.send("first!".to_string()).unwrap();
        assert_eq!(rx1.await.unwrap(), "first!");

        // Remove second globally
        let (key, sender) = bag.remove_first_global().unwrap();
        assert_eq!(key, "job2");
        sender.send("second!".to_string()).unwrap();
        assert_eq!(rx2.await.unwrap(), "second!");

        // Bag is now empty
        assert!(bag.remove_first_global().is_none());
        Ok(())
    }
}
