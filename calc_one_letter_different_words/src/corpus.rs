use std::collections::HashMap;
use std::ops::{AddAssign, Index, IndexMut};

/// Represents all the words we are interested in, organized
/// in a HashMap by their length.
pub struct Corpus {
    pub words: HashMap<usize, WordSet>,
}

/// Represents all the words of a particular length.
pub struct WordSet {
    word_length: usize,
    pub words: Vec<String>,
}

impl Corpus {
    pub fn new() -> Self {
        Corpus {
            words: HashMap::new(),
        }
    }

    pub fn sorted_keys(&self) -> Vec<usize> {
        let mut keys: Vec<_> = self.words.keys().copied().collect();
        keys.sort_unstable();
        keys
    }
}

impl Index<usize> for Corpus {
    type Output = WordSet;

    fn index(&self, word_length: usize) -> &Self::Output {
        &self.words[&word_length]
    }
}

impl IndexMut<usize> for Corpus {
    fn index_mut(&mut self, word_length: usize) -> &mut Self::Output {
        self.words.get_mut(&word_length).unwrap()
    }
}

impl AddAssign<String> for Corpus {
    /// Adds a new word to the corpus in the appropriate slot.
    fn add_assign(&mut self, word: String) {
        let len = word.len();
        let wt = self
            .words
            .entry(len)
            .or_insert_with(|| WordSet::new(len));
        wt.words.push(word);
    }
}

impl WordSet {
    pub fn new(word_length: usize) -> Self {
        Self {
            word_length,
            words: Vec::new(),
        }
    }

    pub fn word_length(&self) -> usize {
        self.word_length
    }
}
