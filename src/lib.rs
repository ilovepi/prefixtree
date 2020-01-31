use std::cell::RefCell;
use std::collections::btree_map::BTreeMap;

struct PrefixTree<T>
where
    T: Ord,
{
    val: T,
    children: BTreeMap<T, RefCell<PrefixTree<T>>>,
}

impl<T> PrefixTree<T>
where
    T: Ord,
{
    fn new(v: T) -> PrefixTree<T> {
        PrefixTree {
            val: v,
            children: BTreeMap::new(),
        }
    }

    fn insert(&mut self, key: &[T]) {
        let i = key.iter();
        self.insert_iter(i);
    }

    fn insert_iter(&mut self, mut i: impl Iterator<Item =& T>) {
        if let Some(val) = i.next() {
            if self.children.contains_key(&val) {
                let child = self.children.get_mut(&val).unwrap();
                child.get_mut().insert_iter(i);
            } else {
                let mut p = PrefixTree {
                    val: val.clone(),
                    children: BTreeMap::new(),
                };
                p.insert_iter(i);
                self.children.insert(val.clone(), p);
            }
        }
    }

    fn search(&self, key: &[T]) {
        let mut i = key.iter();
    }

    fn search_iter(&self, key: T, mut i: impl Iterator<Item = T>) -> bool {
        if self.val == key {
            if self.children.is_empty() {
                return true;
            }
            if let Some(val) = i.next() {
                if self.children.contains_key(val) {
                    return self.children[val].search_iter(i);
                } else {
                    return false;
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::PrefixTree;

    #[test]
    fn it_works() {
        let word = "abcd".chars();
        let p = PrefixTree::new(word);
        assert!(
            p.search(word),
            "Just Inserted Key was missing from PrefixTree"
        )
    }
}
