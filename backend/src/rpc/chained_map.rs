use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct ChainedMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
{
    map: HashMap<K, VecDeque<V>>, // key → queue of values directly
}

impl<K, V> ChainedMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.entry(key).or_default().push_back(value);
    }

    /// Remove and return the first value for a given key
    pub fn remove_first_for_key(&mut self, key: &K) -> Result<Option<V>> {
        if let Some(values) = self.map.get_mut(key) {
            if let Some(value) = values.pop_front() {
                if values.is_empty() {
                    self.map.remove(key);
                }
                return Ok(Some(value));
            }
        }
        Ok(None)
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
    async fn test_race_condition_invalid_index() -> Result<()> {
        let mut bag: ChainedMap<&str, oneshot::Sender<String>> = ChainedMap::new();

        // Insert multiple items with the same key
        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        let (tx3, rx3) = oneshot::channel();

        bag.insert("job1", tx1);
        bag.insert("job1", tx2);
        bag.insert("job1", tx3);

        // Remove the first item by key
        let sender = bag.remove_first_for_key(&"job1")?.unwrap();
        sender.send("first!".to_string()).unwrap();
        assert_eq!(rx1.await.unwrap(), "first!");

        // Remove the second item by key
        let sender = bag.remove_first_for_key(&"job1")?.unwrap();
        sender.send("second!".to_string()).unwrap();
        assert_eq!(rx2.await.unwrap(), "second!");

        // Remove the last one
        let sender = bag.remove_first_for_key(&"job1")?.unwrap();
        sender.send("third!".to_string()).unwrap();
        assert_eq!(rx3.await.unwrap(), "third!");

        // Should be empty now
        assert!(bag.remove_first_for_key(&"job1")?.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_keys_operations() -> Result<()> {
        let mut bag: ChainedMap<&str, oneshot::Sender<String>> = ChainedMap::new();

        // Insert items with different keys
        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        let (tx3, rx3) = oneshot::channel();
        let (tx4, rx4) = oneshot::channel();

        bag.insert("job1", tx1);
        bag.insert("job2", tx2);
        bag.insert("job1", tx3);
        bag.insert("job2", tx4);

        // Remove items by key - should handle multiple keys gracefully
        let sender = bag.remove_first_for_key(&"job1")?.unwrap();
        sender.send("first!".to_string()).unwrap();
        assert_eq!(rx1.await.unwrap(), "first!");

        let sender = bag.remove_first_for_key(&"job2")?.unwrap();
        sender.send("second!".to_string()).unwrap();
        assert_eq!(rx2.await.unwrap(), "second!");

        // Remove remaining items
        let sender = bag.remove_first_for_key(&"job1")?.unwrap();
        sender.send("third!".to_string()).unwrap();
        assert_eq!(rx3.await.unwrap(), "third!");

        let sender = bag.remove_first_for_key(&"job2")?.unwrap();
        sender.send("fourth!".to_string()).unwrap();
        assert_eq!(rx4.await.unwrap(), "fourth!");

        // Should be empty now
        assert!(bag.remove_first_for_key(&"job1")?.is_none());
        assert!(bag.remove_first_for_key(&"job2")?.is_none());
        Ok(())
    }
}
