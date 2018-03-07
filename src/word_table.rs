pub struct WordTable {
    word_length: usize,
    pub words : Vec<String>
}

impl WordTable {
    pub fn new(word_length: usize) -> Self {
        WordTable { word_length, words : Vec::new() }
    }

    pub fn word_length(&self) -> usize {
        self.word_length
    }
}
