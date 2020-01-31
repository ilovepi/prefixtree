use core::borrow::Borrow;
use std::cell::RefCell;
use std::collections::btree_map::BTreeMap;

struct PrefixTreeRoot {
    children: BTreeMap<char, RefCell<PrefixTree>>,
}

impl PrefixTreeRoot {
    fn new() -> PrefixTreeRoot {
        {
            PrefixTreeRoot {
                children: BTreeMap::new(),
            }
        }
    }

    fn create(s: &str) -> Option<PrefixTreeRoot> {
        if let Some(c) = s.chars().next() {
            let mut p = PrefixTreeRoot::new();
            p.insert(s);
            return Some(p);
        }
        return None;
    }

    fn insert(&mut self, s: &str) {
        let mut i = s.chars().into_iter();
        if let Some(val) = i.next() {
            if let Some(child_ref) = self.children.get(&val) {
                let mut child = child_ref.borrow_mut();
                child.insert_iter(i);
            } else {
                if let Some(p) = PrefixTree::create(s) {
                    self.children.insert(val, RefCell::new(p));
                }
            }
        }
    }

    fn search(&self, key: &str) -> bool {
        let mut i = key.chars().into_iter();
        if let Some(val) = i.next() {
            return self.children.get(&val).unwrap().borrow().search_iter(&val, i);
        }
        return false;
    }

    fn search_iter<'a>(&self, key: &char, mut i: impl Iterator<Item = char>) -> bool {
        if let Some(val) = i.next() {
            if self.children.contains_key(&val) {
                let child = self.children.get(&val).unwrap();
                return child.borrow().search_iter(&val, i);
            } else {
                return false;
            }
        }
        return false;
    }

    fn print(&self) -> Vec<String> {
        let mut ret = Vec::new();
        for (_, cell) in &self.children {
            let child = cell.borrow();
            let t = child.print();
            for word in t {
                ret.push(word);
            }
        }
        return ret;
    }
}

struct PrefixTree {
    val: char,
    children: BTreeMap<char, RefCell<PrefixTree>>,
}

impl PrefixTree {
    fn new(v: char) -> PrefixTree {
        PrefixTree {
            val: v,
            children: BTreeMap::new(),
        }
    }

    fn create(s: &str) -> Option<PrefixTree> {
        if let Some(c) = s.chars().next() {
            let mut p = PrefixTree::new(c);
            p.insert(s.get(1..).unwrap());
            return Some(p);
        }
        return None;
    }

    fn insert(&mut self, key: &str) {
        let i = key.chars();
        self.insert_iter(i);
    }

    fn insert_iter(&mut self, mut i: impl Iterator<Item = char>) {
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
                self.children.insert(val.clone(), RefCell::new(p));
            }
        }
    }

    fn search(&self, key: &str) -> bool {
        let mut i = key.chars().into_iter();
        if let Some(val) = i.next() {
            return self.search_iter(&val, i);
        }
        return false;
    }

    fn search_iter<'a>(&self, key: &char, mut i: impl Iterator<Item = char>) -> bool {
        if self.val == *key {
            if self.children.is_empty() {
                return true;
            }
            if let Some(val) = i.next() {
                if self.children.contains_key(&val) {
                    let child = self.children.get(&val).unwrap();
                    return child.borrow().search_iter(&val, i);
                } else {
                    return false;
                }
            }
        }
        return false;
    }

    fn print(&self) -> Vec<String> {
        let mut ret = Vec::new();

        let mut o: String = String::new();
        o.push(self.val);
        if self.children.is_empty() {
            ret.push(o);
            return ret;
        }

        for (_, cell) in &self.children {
            let child = cell.borrow();
            let t = child.print();
            for word in t {
                ret.push(o.clone() + &word);
            }
        }

        return ret;
    }
}

#[cfg(test)]
mod tests {
    use crate::{PrefixTree, PrefixTreeRoot};

    #[test]
    fn it_works() {
        let word = "abcd";
        let mut p = PrefixTreeRoot::create(word).unwrap();
        p.insert(word);
        p.insert("foobar");
        for word in p.print() {
            println!("{}", word);
        }
        assert!(
            p.search(word),
            "Just Inserted Key was missing from PrefixTree"
        )
    }
}
