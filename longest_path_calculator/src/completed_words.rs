use std::collections::HashMap;

use graph::RelativeDirectories;

#[derive(Default)]
pub struct CompletedWords {
    completed: HashMap<usize, Vec<String>>,
}

pub fn get_completed_words(dirs: &RelativeDirectories, word_lengths: &[usize]) -> CompletedWords {
    let mut completed_words = CompletedWords::default();

    for word_length in word_lengths {
        let dir = dirs.chains_directory(*word_length);
        if let Ok(dir) = dir.read_dir() {
            for dir_entry in dir {
                if let Ok(file) = dir_entry {
                    let filename = file.path();
                    let filename = filename.file_stem().unwrap();
                    let filename = filename.to_string_lossy().into_owned();

                    let entry = completed_words
                        .completed
                        .entry(*word_length)
                        .or_insert_with(|| Vec::<String>::new());
                    entry.push(filename);
                }
            }
        }
    }

    completed_words
}

impl CompletedWords {
    /// Returns the number of words complete of the specified length.
    pub fn num_complete(&self, word_length: usize) -> usize {
        if let Some(v) = self.completed.get(&word_length) {
            v.len()
        } else {
            0
        }
    }
    
    /// Get a slice of the words completed by length.
    pub fn completed_words_of_length(&self, word_length: usize) -> &[String] {
        if let Some(v) = self.completed.get(&word_length) {
            v
        } else {
            &[]
        }
    }
}
