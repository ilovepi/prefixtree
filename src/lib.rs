use core::borrow::Borrow;
use std::cell::RefCell;
use std::collections::btree_map::BTreeMap;

struct PrefixTree {
    val: char,
    children: BTreeMap<char, RefCell<PrefixTree>>,
}

impl PrefixTree {
    fn create(s: &str) -> Option<PrefixTree> {
        if let Some(c) = s.chars().next() {
            let mut p = PrefixTree::new(c);
            p.insert(s.get(1..)?);
            return Some(p);
        }
        return None;
    }

    fn new(v: char) -> PrefixTree {
        PrefixTree {
            val: v,
            children: BTreeMap::new(),
        }
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
                ret.push( o.clone() + &word);
            }
        }

        return ret;
    }
}

#[cfg(test)]
mod tests {
    use crate::PrefixTree;

    #[test]
    fn it_works() {
        let word = "abcd";
        let p = PrefixTree::create(word).unwrap();
        for word in p.print(){
            println!("{}", word);
        }
        assert!(
            p.search(word),
            "Just Inserted Key was missing from PrefixTree"
        )
    }
}
