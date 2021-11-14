use std::{collections::HashSet, path::Path};
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufRead};

/// Reads all the available input dictionaries, filters the words for basic acceptability, and
/// then creates a single merged dictionary called "corpus.txt" in the output folder.
pub(crate) fn merge_dictionaries(dictionary_dir: &Path, corpus_file: &Path) {
    let mut words = HashSet::new();
    let paths = fs::read_dir(dictionary_dir).expect("Could not read entries in dictionaries folder");

    println!("Merging dictionary files from {:?} to {:?}", dictionary_dir, corpus_file);
    for path in paths {
        let path = path.unwrap().path();
        println!("Reading words from {:?}", path);

        let num_words_at_start = words.len();
        let file = fs::File::open(path).expect("no such file");
        let rdr = io::BufReader::new(file);

        let mut num_words_read_from_file = 0;
        for bytes in rdr.split(b'\n').flatten() {
            num_words_read_from_file += 1;

            if let Ok(word) = String::from_utf8(bytes) {
                if let Some(w) = clean_word(word) {
                    words.insert(w);
                }
            }
        }

        let num_words_added = words.len() - num_words_at_start;
        println!(
            "    Read {} words and added {}",
            num_words_read_from_file, num_words_added
        );
    }

    write_corpus_file(words, corpus_file);
}

fn clean_word(w: String) -> Option<String> {
    if w.len() <= 2 {
        return None;
    }

    let w = w.to_lowercase().trim().to_string();
    if w.chars().all(|c| c.is_ascii_lowercase()) {
        Some(w)
    } else {
        None
    }
}

fn write_corpus_file(words: HashSet<String>, corpus_file: &Path) {
    let parent = corpus_file.parent().unwrap();
    fs::create_dir_all(parent).unwrap();
    let out_file = fs::File::create(corpus_file).expect("Unable to create corpus_file");
    let mut writer = io::BufWriter::new(out_file);
    let mut words: Vec<_> = words.into_iter().collect();
    words.sort_unstable();
    for w in &words {
        writeln!(writer, "{}", w).unwrap();
    }

    println!("Finished, wrote {} words to {:?}", words.len(), corpus_file);
}

