use std::collections::HashMap;
use std::ops::{Index, IndexMut, AddAssign};
use ::word_table::WordTable;

pub struct WordBase {
    pub words: HashMap<usize, WordTable>
}

impl WordBase {
    pub fn new() -> Self {
        WordBase { words: HashMap::new() }
    }

    pub fn sorted_keys(&self) -> Vec<usize> {
        let mut keys : Vec<usize> = self.words.keys().map(|&k| k).collect();
        keys.sort();
        keys
    }
}

impl Index<usize> for WordBase {
    type Output = WordTable;

    fn index(&self, word_length: usize) -> &WordTable {
        &self.words[&word_length]
    }
}

impl IndexMut<usize> for WordBase {
    fn index_mut(&mut self, word_length: usize) -> &mut WordTable {
        self.words.get_mut(&word_length).unwrap()
    }
}

impl AddAssign<String> for WordBase {
    fn add_assign(&mut self, word: String) {
        let len = word.len();
        let wt = self.words.entry(len).or_insert(WordTable::new(len));
        wt.words.push(word);
    }
}