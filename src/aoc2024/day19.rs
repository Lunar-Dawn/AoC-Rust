use std::collections::{BTreeMap, HashMap};

use crate::runner;
use crate::util::DynResult;

runner!();

#[derive(Eq, Hash, PartialEq)]
struct TrieNode {
    is_leaf: bool,
    children: BTreeMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            is_leaf: false,
            children: BTreeMap::new(),
        }
    }
    fn insert(&mut self, word: &str) {
        if word.is_empty() {
            self.is_leaf = true;
            return;
        }

        let child = self
            .children
            .entry(word.chars().next().unwrap())
            .or_insert(TrieNode::new());
        child.insert(&word[1..]);
    }

    fn search_root<'a>(&'a self, pattern: &'a str, memory: &mut HashMap<&'a str, u64>) -> u64 {
        if let Some(b) = memory.get(&(pattern)) {
            return *b;
        }

        let mut ret = 0;

        let c = pattern.chars().next().unwrap();
        if let Some(child) = self.children.get(&c) {
            ret += child.search(&pattern[1..], &self, memory)
        }

        memory.insert(pattern, ret);
        ret
    }

    fn search<'a>(
        &self,
        pattern: &'a str,
        root: &'a Self,
        memory: &mut HashMap<&'a str, u64>,
    ) -> u64 {
        if pattern.is_empty() {
            return self.is_leaf as u64;
        }

        let mut ret = 0;

        if self.is_leaf {
            ret += root.search_root(pattern, memory)
        }

        let c = pattern.chars().next().unwrap();
        if let Some(child) = self.children.get(&c) {
            ret += child.search(&pattern[1..], root, memory)
        }

        ret
    }
}

type ParsedData = (u64, u64);
fn parse(input: &str) -> DynResult<ParsedData> {
    let mut lines = input.lines();

    let trie_line = lines.next().unwrap();
    let mut root = TrieNode::new();

    trie_line.split(", ").for_each(|towel| root.insert(towel));

    let mut num_possible = 0;
    let mut sum = 0;

    let mut cache = HashMap::new();

    for pattern in lines.skip(1) {
        let res = root.search_root(pattern, &mut cache);
        num_possible += (res > 0) as u64;
        sum += res;
    }

    Ok((num_possible, sum))
}

fn part1((num_possible, _): &ParsedData) -> u64 {
    *num_possible
}
fn part2((_, sum): &ParsedData) -> u64 {
    *sum
}
