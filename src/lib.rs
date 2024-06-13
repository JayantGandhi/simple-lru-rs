use std::collections::HashMap;
use std::cell::RefCell;
use std::hash::Hash;

struct LruNode<K, V> {
    key: K,
    value: V,
    prev: Option<K>,
    next: Option<K>
}

impl<K, V> LruNode<K, V> {
    fn new(key: K, value: V) -> Self {
        LruNode {
            key,
            value,
            prev: None,
            next: None
        }
    }
}

impl<K, V> Clone for LruNode<K, V>
where
    K: Clone,
    V: Clone
{
    fn clone(&self) -> Self {
        LruNode {
            key: self.key.clone(),
            value: self.value.clone(),
            prev: self.prev.clone(),
            next: self.next.clone()
        }
    }
}

pub struct LruCache<K: Clone + Eq + Hash, V> {
    capacity: usize,
    map: HashMap<K, RefCell<LruNode<K, V>>>,
    head: Option<K>,
    tail: Option<K>
}

impl<K: Clone + Eq + Hash, V: Clone> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: None
        }
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        let value = match self.map.get_mut(&key) {
            None =>  return None,
            Some(node_ref) => {
                let node = node_ref.borrow();
                node.value.clone()
            }
        };
    
        self.move_to_back(&key);
        Some(value)
    }

    fn move_to_back(&mut self, key: &K) {
        if let Some(node_ref) = self.map.get(&key).cloned() {
            self.remove_node(&node_ref);
            self.append_node(&node_ref);
        }
    }


    pub fn put(&mut self, key: K, value: V) {
        if let Some(node_ref) = self.map.get(&key).cloned() {
            let mut node = node_ref.borrow_mut();
            node.value = value;
            drop(node); // Explicitly drop the mutable borrow
            self.remove_node(&node_ref);
            self.append_node(&node_ref);
        } else {
            if self.map.len() == self.capacity {
                if let Some(head_ref) = self.head.as_ref().cloned() {
                    self.evict_node(&head_ref)
                }
            }
    
            let node = LruNode::new(key.clone(), value);
            let node_ref = RefCell::new(node);
            self.map.insert(key.clone(), node_ref.clone());
            self.append_node(&node_ref);
        }

    }

    fn evict_node (&mut self, key: &K) {
        let node = self.map.get(key).unwrap().clone();
        self.remove_node(&node);
        self.map.remove(key);
    }

    fn remove_node(&mut self, node: &RefCell<LruNode<K, V>>) {
        let (prev_ref, next_ref) = {
            let node_borrow = node.borrow();
            (node_borrow.prev.clone(), node_borrow.next.clone())
        };

        match prev_ref.clone() {
            None => {
                self.head = next_ref.clone();
            },
            Some(prev_ref) => {
                self.map.get(&prev_ref).unwrap().borrow_mut().next = next_ref.clone();
            }
        }
    
        match next_ref {
            None => {
                self.tail = prev_ref.clone();
            },
            Some(next_ref) => {
                self.map.get(&next_ref).unwrap().borrow_mut().prev = prev_ref;
            }
        }
    }

    fn append_node(&mut self, node: &RefCell<LruNode<K, V>>) {
        match self.tail {
            None => {
                let key = node.borrow().key.clone();
                self.head = Some(key.clone());
                self.tail = Some(key.clone());
            },
            Some(_) => {
                let mut node = node.borrow_mut();
                node.prev = self.tail.clone();

                let mut tail_node = self.map.get(self.tail.as_ref().unwrap()).unwrap().borrow_mut();
                tail_node.next = Some(node.key.clone());

                self.tail = Some(node.key.clone());
            }
        }
    }

    pub fn delete(&mut self, key: K) {
        if let Some(node_ref) = self.map.get(&key).cloned() {
            self.remove_node(&node_ref);
            self.map.remove(&key);
        }
    }

    pub fn reset(&mut self) {
        self.map.clear();
        self.head = None;
        self.tail = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = LruCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), Some(1));
        cache.put(3, 3);
        assert_eq!(cache.get(2), None);
        cache.put(4, 4);
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(3), Some(3));
        assert_eq!(cache.get(4), Some(4));
    }

    #[test]
    fn test_lru_cache_delete() {
        let mut cache = LruCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.delete(1);
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(2));
    }

    #[test]
    fn test_lru_cache_reset() {
        let mut cache = LruCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.reset();
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), None);
    }

    #[test]
    fn test_lru_with_complex_values() {
        let mut cache = LruCache::new(2);
        cache.put("a", vec![1, 2, 3]);
        cache.put("b", vec![4, 5, 6]);
        assert_eq!(cache.get("a"), Some(vec![1, 2, 3]));
        cache.put("c", vec![7, 8, 9]);
        assert_eq!(cache.get("b"), None);
        cache.put("d", vec![10, 11, 12]);
        assert_eq!(cache.get("a"), None);
        assert_eq!(cache.get("c"), Some(vec![7, 8, 9]));
        assert_eq!(cache.get("d"), Some(vec![10, 11, 12]));
    }
}