use std::collections::hash_map::{Entry, HashMap};

#[derive(Default, Debug)]
/// Preserves insertion order.
/// Later insertions will be ignored!
pub struct Map {
    map: HashMap<String, String>,
    inserted: Vec<String>,
}

impl Map {
    /// returns if it was new or not
    fn insert(&mut self, key: String, val: String) -> bool {
        match self.map.entry(key.clone()) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                e.insert(val.clone());
                self.inserted.push(key);
                true
            }
        }
    }
}

impl FromIterator<(String, String)> for Map {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut map = Map::default();

        iter.into_iter().for_each(|(k, v)| {
            map.insert(k, v);
        });
        map
    }
}

pub struct Iter {
    vec_iter: std::vec::IntoIter<String>,
    map: HashMap<String, String>,
}

impl Iterator for Iter {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.vec_iter.next().map(|k| {
            let val = self.map.remove(&k).unwrap();
            (k, val)
        })
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.vec_iter.next_back().map(|k| {
            let val = self.map.remove(&k).unwrap();
            (k, val)
        })
    }
}

impl ExactSizeIterator for Iter {
    fn len(&self) -> usize {
        self.vec_iter.len()
    }
}

impl IntoIterator for Map {
    type Item = (String, String);

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            vec_iter: self.inserted.into_iter(),
            map: self.map,
        }
    }
}
