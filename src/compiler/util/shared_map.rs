use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

/// A shared mutable hash map managed by reference counting.
/// 
/// # Cloning
/// 
/// The `Clone` trait implements cloning of the map by reference.
/// Use the `clone_content()` method to clone the map by content.
#[derive(Clone)]
pub struct SharedMap<K, V>(Rc<HashMap<K, V>>);

impl<K, V> PartialEq for SharedMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<K, V> Eq for SharedMap<K, V> {}

impl<K, V> SharedMap<K, V> {
    pub fn new() -> Self {
        Self(Rc::new(HashMap::new()))
    }

    pub fn get(&self, key: &K) -> Option<V> where K: Eq + Hash, V: Clone {
        self.0.get(key).map(|v| v.clone())
    }

    pub fn set(&mut self, key: K, value: V) where K: Eq + Hash {
        Rc::get_mut(&mut self.0).unwrap().insert(key, value);
    }

    pub fn remove(&mut self, key: &K) -> Option<V> where K: Eq + Hash {
        Rc::get_mut(&mut self.0).unwrap().remove(key)
    }

    pub fn has(&self, key: &K) -> bool where K: Eq + Hash {
        self.0.contains_key(key)
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> SharedMapIterator<K, V> {
        SharedMapIterator(self.0.iter())
    }

    pub fn clone_content(&self) -> Self where K: Clone + Eq + Hash, V: Clone {
        let mut r = Self::new();
        for (k, v) in self.iter() {
            r.set(k.clone(), v.clone());
        }
        r
    }
}

pub struct SharedMapIterator<'a, K, V>(Iter<'a, K, V>);

impl<'a, K, V> Iterator for SharedMapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<K: Eq + Hash, V> FromIterator<(K, V)> for SharedMap<K, V> {
    fn from_iter<T2: IntoIterator<Item = (K, V)>>(iter: T2) -> Self {
        let mut r = Self::new();
        for (k, v) in iter {
            r.set(k, v);
        }
        r
    }
}