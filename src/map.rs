//! This module implements a key-value `Map` with an API
//! similar to [std::collections::HashMap].

use std::{
    borrow::Borrow,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Clone)]
/// A simple hash map implementation.
/// This uses a separate chaining strategy to handle collisions.
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

/// The default initial capacity of the `HashMap`.
const INITIAL_CAPACITY: usize = 1 << 4;
/// The percentage of fullness we need inside the
/// `HashMap` before resizing it.
const LOAD_FACTOR: f64 = 0.7;

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            // we do that instead of Vec::with_capacity(INITIAL_CAPACITY)
            // cause we ensure that the buckets will be initialised with empty vectors
            // and not with garbage values
            buckets: (0..INITIAL_CAPACITY).map(|_| Vec::new()).collect(),
            len: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.should_resize() {
            self.resize();
        }

        let idx = self.bucket_index(&key);
        // update existing key if found
        for entry in &mut self.buckets[idx] {
            if entry.0 == key {
                // this replaces the old entry value with the new one
                // and returns the old value, very convenient :D
                let old = std::mem::replace(&mut entry.1, value);
                return Some(old);
            }
        }

        self.buckets[idx].push((key, value));
        self.len += 1;
        None
    }

    pub fn get<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
    {
        let idx = self.bucket_index(key);
        /// linear search inside the bucket for the key match
        /// we prob could do a binary search but I don't think the
        /// bucket will be that big given the data
        self.buckets[idx]
            .iter()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    fn bucket_index<Q: ?Sized + Hash>(&self, key: &Q) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.buckets.len()
    }

    #[inline(always)]
    fn should_resize(&self) -> bool {
        self.len as f64 / self.buckets.len() as f64 > LOAD_FACTOR
    }

    fn resize(&mut self) {
        let new_capacity = self.buckets.len() * 2;
        let mut new_buckets: Vec<Vec<(K, V)>> = (0..new_capacity).map(|_| Vec::new()).collect();

        for bucket in self.buckets.drain(..) {
            // we need to rehash every key because the capacity changed
            // and the bucket index depends on the capacity
            //
            // this is not very efficient but it works :D
            for (key, value) in bucket {
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                let idx = hasher.finish() as usize % new_capacity;
                new_buckets[idx].push((key, value));
            }
        }

        self.buckets = new_buckets;
    }
}

impl<K, Q, V> std::ops::Index<&Q> for HashMap<K, V>
where
    K: Hash + Eq + std::borrow::Borrow<Q>,
    Q: ?Sized + Hash + Eq,
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).expect("key was not found")
    }
}

#[cfg(test)]
mod test {
    use super::HashMap;

    #[test]
    /// test from: <https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert>
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(map.insert(37, "a"), None);
        assert_eq!(map.is_empty(), false);

        map.insert(37, "b");
        assert_eq!(map.insert(37, "c"), Some("b"));
        assert_eq!(map[&37], "c");
    }

    #[test]
    fn get_none() {
        let mut map: HashMap<usize, usize> = HashMap::new();
        assert_eq!(map.get(&420), None);
    }

    #[test]
    /// test from: <https://app.studyraid.com/en/read/11458/359168/documenting-and-testing-hashmap-implementations>
    fn bunk_insert() {
        let mut map = HashMap::new();

        for idx in 0..10000 {
            map.insert(idx, idx);
        }

        assert_eq!(map.get(&9999), Some(&9999));
        assert_eq!(map.len(), 10000);
    }

    #[test]
    /// test from: <https://gist.github.com/seadowg/1431727/c81a08e59543f7a5da646467cd413cc6200405ba>
    fn correct_returned_value() {
        let mut map = HashMap::new();
        map.insert("Hello", 5);
        map.insert("Goodbye", 6);

        assert_eq!(map.get("Hello"), Some(&5));
        assert_eq!(map.get("Goodbye"), Some(&6));
    }
}
