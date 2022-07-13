use std::collections::HashSet;

#[derive(Default)]
pub struct Set {
    set: HashSet<String>,
    inserted: Vec<String>,
}

impl Set {
    fn insert(&mut self, val: String) -> bool {
        let is_new = self.set.insert(val.clone());

        if is_new {
            self.inserted.push(val);
        }

        is_new
    }
}

impl FromIterator<String> for Set {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut set = Set::default();

        iter.into_iter().for_each(|s| {
            set.insert(s);
        });
        set
    }
}

pub struct Iter(std::vec::IntoIter<String>);

impl Iterator for Iter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl ExactSizeIterator for Iter {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for Set {
    type Item = String;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.inserted.into_iter())
    }
}
