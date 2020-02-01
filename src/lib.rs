extern crate log;
use log::{debug, error, info};
use std::cell::RefCell;
use std::collections::btree_map::BTreeMap;

pub struct PrefixTreeRoot {
    children: BTreeMap<char, RefCell<PrefixTree>>,
}

impl PrefixTreeRoot {
    pub fn new() -> PrefixTreeRoot {
        {
            PrefixTreeRoot {
                children: BTreeMap::new(),
            }
        }
    }

    pub fn create(s: &str) -> Option<PrefixTreeRoot> {
        if s.is_empty() {
            return None;
        }
        let mut p = PrefixTreeRoot::new();
        p.insert(s);
        return Some(p);
    }

    pub fn insert(&mut self, s: &str) {
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

    pub fn search(&self, key: &str) -> bool {
        let i = key.chars().into_iter();
        self.search_iter(i)
    }

    fn search_iter<'a>(&self, mut i: impl Iterator<Item = char> + Clone) -> bool {
        let mut j = i.clone().peekable();
        if let Some(val) = i.next() {
            info!("Searching for val: {}", val);
            match self.children.get(&val) {
                Some(child) => return child.borrow().search_iter(&val, i),
                None => return false,
            }
        }
        error!("Search key was empty.");
        return false;
    }

    fn search_prefix_iter<'a>(&self, mut i: impl Iterator<Item = char> + Clone) -> bool {
        let mut j = i.clone().peekable();
        if let Some(val) = i.next() {
            info!("Searching for val: {}", val);
            match self.children.get(&val) {
                Some(child) => return child.borrow().search_prefix(&val, i),
                None => return false,
            }
        }
        error!("Search key was empty.");
        return true; // empty string is always a prefix
    }

    pub fn dump(&self) -> Vec<String> {
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

    pub fn print(&self) {
        for word in self.dump() {
            println!("{}", word)
        }
    }

    pub fn debug(&self) {
        self.dump().iter().map(|v| info!("{}", v)).collect()
    }

    pub fn starts_with(&self, s: &str) -> bool {
        let mut i = s.chars().into_iter();
        self.search_prefix_iter(i)
    }
}

pub struct PrefixTree {
    val: char,
    children: BTreeMap<char, RefCell<PrefixTree>>,
    is_end: bool,
}

impl PrefixTree {
    pub fn new(v: char, end: bool) -> PrefixTree {
        PrefixTree {
            val: v,
            children: BTreeMap::new(),
            is_end: end,
        }
    }

    pub fn create(s: &str) -> Option<PrefixTree> {
        if let Some(c) = s.chars().next() {
            let mut p = PrefixTree::new(c, s.len() == 1);
            p.insert(s.get(1..).unwrap());
            return Some(p);
        }
        return None;
    }

    pub fn insert(&mut self, key: &str) {
        let i = key.chars();
        self.insert_iter(i);
    }

    fn insert_iter(&mut self, mut i: impl Iterator<Item = char>) {
        match i.next() {
            Some(val) => match self.children.get_mut(&val) {
                Some(child) => {
                    child.get_mut().insert_iter(i);
                }
                None => {
                    // we need to insert new children
                    let mut p = PrefixTree {
                        val: val.clone(),
                        children: BTreeMap::new(),
                        is_end: false,
                    };
                    if !p.is_end {
                        p.insert_iter(i);
                    }
                    self.children.insert(val.clone(), RefCell::new(p));
                }
            },
            None => {
                // iterator is exhausted we're at the end of a word
                self.is_end = true;
            }
        }
        debug!("insert_iter -- val: {}, is_end: {}", self.val, self.is_end);
    }

    pub fn search(&self, key: &str) -> bool {
        let mut i = key.chars().into_iter();
        let mut j = i.clone().peekable();
        if let Some(val) = j.peek() {
            return self.search_iter(&val, i);
        }
        return false;
    }

    fn search_iter<'a>(&self, key: &char, mut i: impl Iterator<Item = char>) -> bool {
        if self.val != *key {
            info!("Key not found. val: {}", key);
            return false;
        }

        info!("Currently in node '{}'", self.val);
        match i.next() {
            Some(val) => match self.children.get(&val) {
                Some(child) => {
                    info!("search found val: {}", val);
                    return child.borrow().search_iter(&val, i);
                }
                None => {
                    info!("no matching children for val: {}", val);
                    // iterator still has elements, and we failed to match
                    return false;
                }
            },
            None => {
                info!("Out of chars, are we done? {}", self.is_end);
                // iterator is exhausted check if we're at the end of a word
                return self.is_end;
            }
        }
    }

    fn search_prefix<'a>(&self, key: &char, mut i: impl Iterator<Item = char>) -> bool {
        if self.val != *key {
            info!("Key not found. val: {}", key);
            return false;
        }

        info!("Currently in node '{}'", self.val);
        match i.next() {
            Some(val) => match self.children.get(&val) {
                Some(child) => {
                    info!("search found val: {}", val);
                    return child.borrow().search_prefix(&val, i);
                }
                None => {
                    info!("no matching children for val: {}", val);
                    // iterator still has elements, and we failed to match
                    return false;
                }
            },
            None => {
                info!("Out of chars, prefix found.");
                // iterator is exhausted check if we're at the end of a word
                return true;
            }
        }
    }

    pub fn print(&self) -> Vec<String> {
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
    use super::*;
    use simple_logger;
    use slog_envlogger;
    use slog_stdlog;
    //use crate::{PrefixTree, PrefixTreeRoot};

    fn init_log() {
        //let _ = env_logger::builder().is_test(true).try_init();
        //let _ = simple_logger::init();
        let _ = pretty_env_logger::try_init();
        //let _guard = slog_envlogger::init();
    }

    #[test]
    fn prefix_tree_create_test() {
        init_log();
        let word = "abcd";
        let p = PrefixTreeRoot::create(word).unwrap();
        for word in p.dump() {
            info!("{}", word);
        }
        assert!(
            p.search(word),
            "Just Inserted Key was missing from PrefixTree"
        );
    }

    #[test]
    fn prefix_tree_insert_test() {
        init_log();
        let word = "abcd";
        let mut p = PrefixTreeRoot::create(word).unwrap();
        p.insert("foobar");
        for word in p.dump() {
            info!("{}", word);
        }

        assert!(
            p.search(word),
            "Just Inserted Key was missing from PrefixTree"
        );

        assert!(
            p.search("foobar"),
            "Just Inserted Key was missing from PrefixTree"
        );
    }

    #[test]
    fn prefix_tree_search_prefix_returns_false() {
        init_log();
        let word = "abcd";
        let mut p = PrefixTreeRoot::create(word).unwrap();
        p.insert("foobar");
        p.debug();
        assert!(!p.search("abc"), "Found a prefix but returned a match");
    }
    #[test]
    fn prefix_tree_starts_with_returns_true() {
        init_log();
        let word = "abcd";
        let mut p = PrefixTreeRoot::create(word).unwrap();
        p.insert("foobar");
        p.debug();
        assert!(p.starts_with("abc"), "Prefix not corectly matched");
        assert!(p.starts_with("foo"), "Prefix not corectly matched");
    }

    #[test]
    fn prefix_tree_search_missing_word_returns_false() {
        init_log();
        let word = "abcd";
        let p = PrefixTreeRoot::create(word).unwrap();
        p.debug();
        assert!(
            !p.search("foobar"),
            "Found a word not inserted into prefix tree"
        );
    }
    #[test]
    fn prefix_tree_search_empty_str_returns_false() {
        init_log();
        let word = "abcd";
        let p = PrefixTreeRoot::create(word).unwrap();
        p.debug();
        assert!(!p.search(""), "Found a word not inserted into prefix tree");
    }
}
