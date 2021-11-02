use std::collections::HashMap;
use std::ops::{AddAssign, Index, IndexMut};
use word_table::WordTable;

pub struct WordBase {
    pub words: HashMap<usize, WordTable>,
}

impl WordBase {
    pub fn new() -> Self {
        WordBase {
            words: HashMap::new(),
        }
    }

    pub fn sorted_keys(&self) -> Vec<usize> {
        let mut keys: Vec<_> = self.words.keys().copied().collect();
        keys.sort_unstable();
        keys
    }
}

impl Index<usize> for WordBase {
    type Output = WordTable;

    fn index(&self, word_length: usize) -> &Self::Output {
        &self.words[&word_length]
    }
}

impl IndexMut<usize> for WordBase {
    fn index_mut(&mut self, word_length: usize) -> &mut Self::Output {
        self.words.get_mut(&word_length).unwrap()
    }
}

impl AddAssign<String> for WordBase {
    fn add_assign(&mut self, word: String) {
        let len = word.len();
        let wt = self.words.entry(len).or_insert_with(|| WordTable::new(len));
        wt.words.push(word);
    }
}
