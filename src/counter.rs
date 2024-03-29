use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
    ops::Add,
};

pub trait Counter<K, V> {
    fn count(&mut self, key: K, value: V) -> V;
}
impl<K, V, S: BuildHasher> Counter<K, V> for HashMap<K, V, S>
where
    K: Hash + Eq,
    V: Add<V, Output = V> + Copy,
{
    fn count(&mut self, key: K, value: V) -> V {
        if self.contains_key(&key) {
            let value = *self.get(&key).unwrap() + value;
            self.insert(key, value);
        } else {
            self.insert(key, value);
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test() {
        let mut counter: HashMap<&str, i64> = HashMap::new();
        counter.count("foo", 1);
        counter.count("bar", 2);
        counter.count("foo", 3);
        counter.count("baz", -1);
        counter.count("bar", 10);
        counter.count("foo", 2);

        assert_eq!(*counter.get("foo").unwrap(), 6);
        assert_eq!(*counter.get("bar").unwrap(), 12);
        assert_eq!(*counter.get("baz").unwrap(), -1);
    }
}
