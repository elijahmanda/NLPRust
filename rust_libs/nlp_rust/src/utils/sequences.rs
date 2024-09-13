use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::Hasher;

pub fn missing_indexes(indexes: Vec<(usize, usize)>, total: usize) -> Vec<(usize, usize)> {
    let mut missing = Vec::new();
    
    if indexes.is_empty() {
        return vec![(0, total)];
    }

    if indexes[0].0 > 0 {
        missing.push((0, indexes[0].0));
    }

    if indexes[indexes.len() - 1].1 < total {
        missing.push((indexes[indexes.len() - 1].1, total));
    }
    for i in 0..indexes.len() - 1 {
        let first_end = indexes[i].1;
        let next_start = indexes[i + 1].0;

        if next_start > first_end {
            missing.push((first_end, next_start));
        }
    }

    missing.sort();
    missing
}

pub fn flatten_sequences<T>(sequences: Vec<Vec<T>>) -> Vec<T> {
    sequences.into_iter().flatten().collect()
}

pub fn count_tokens<T>(sequences: Vec<Vec<T>>) -> usize {
    sequences.iter().map(|seq| seq.len()).sum()
}

pub struct AllEqual<T> {
    _values: Vec<T>,
}


impl<T: std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash> AllEqual<T> {

    pub fn new(values: Vec<T>) -> Self {
        AllEqual { _values: values }
    }

    fn __eq__(&self, other: T) -> bool {
        self._values.contains(&other.into())
    }

    fn __hash__(&self) -> u64 {
        let mut set: HashSet<_> = HashSet::new();
        for value in &self._values {
            set.insert(value);
        }
        std::collections::hash_map::DefaultHasher::new().finish()
    }

    fn __contains__(&self, value: T) -> bool {
        self._values.contains(&value.into())
    }
}

pub fn reverse_dict<K, V>(d: &HashMap<K, V>) -> HashMap<V, K>
where
    K: Eq + std::hash::Hash + Clone,
    V: Eq + std::hash::Hash + Clone,
{
    d.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}
