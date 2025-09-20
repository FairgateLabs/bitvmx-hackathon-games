use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct ChainedMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
{
    map: HashMap<K, Vec<V>>, // key → queue of values directly
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
        self.map.entry(key).or_default().push(value);
    }

    /// Remove and return all values for a given key
    pub fn drain_all_for_key(&mut self, key: &K) -> Result<Vec<V>> {
        if let Some(values) = self.map.remove(key) {
            Ok(values.into())
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tokio::sync::oneshot;

    #[tokio::test]
    async fn test_drain_all_for_key() -> Result<()> {
        let mut bag: ChainedMap<&str, oneshot::Sender<String>> = ChainedMap::new();

        // Insert multiple items with the same key
        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        let (tx3, rx3) = oneshot::channel();

        bag.insert("job1", tx1);
        bag.insert("job1", tx2);
        bag.insert("job1", tx3);

        // Drain all items for the key
        let senders = bag.drain_all_for_key(&"job1")?;
        assert_eq!(senders.len(), 3);

        // Send messages to all drained senders
        let mut senders_iter = senders.into_iter();
        senders_iter
            .next()
            .unwrap()
            .send("first!".to_string())
            .unwrap();
        senders_iter
            .next()
            .unwrap()
            .send("second!".to_string())
            .unwrap();
        senders_iter
            .next()
            .unwrap()
            .send("third!".to_string())
            .unwrap();

        // Verify all messages were received
        assert_eq!(rx1.await.unwrap(), "first!");
        assert_eq!(rx2.await.unwrap(), "second!");
        assert_eq!(rx3.await.unwrap(), "third!");

        // Should be empty now
        assert!(bag.drain_all_for_key(&"job1")?.is_empty());
        Ok(())
    }
}
