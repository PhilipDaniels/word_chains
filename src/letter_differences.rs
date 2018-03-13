use std::fs;
use std::io::{self, BufRead};
use std::io::prelude::*;

use ::dictionary;
use ::word_base::WordBase;
use ::word_table::WordTable;

struct AnchoredWords {
    anchor: String,
    reachable_words: Vec<String>
}

impl AnchoredWords {
    fn new(anchor: String) -> Self {
        AnchoredWords { anchor: anchor, reachable_words: Vec::new() }
    }

    fn add_reachable_word(&mut self, word: String) {
        self.reachable_words.push(word);
    }
}

pub fn calculate_words_one_letter_different() {
    let wordbase = get_words_by_length();

    let keys = wordbase.sorted_keys();
    for key in keys {
        let word_table = &wordbase[key];
        if word_table.words.len() <= 2 { continue };
        let rwords_for_table = calc_reachable_words_for_table(&word_table);
        // This is the initial difference file, includes 2-islands.
        write_difference_file(word_table.word_length(), &rwords_for_table);
    }
}

fn get_words_by_length() -> WordBase {
    let f = fs::File::open(dictionary::CORPUS).unwrap();
    let rdr = io::BufReader::new(f);

    let mut wordbase = WordBase::new();

    println!("Start reading {}", dictionary::CORPUS);

    for word in rdr.lines() {
        let word = word.unwrap();
        wordbase += word;
    }

    println!("Finished reading {}", dictionary::CORPUS);

    let keys = wordbase.sorted_keys();

    for key in keys {
        println!("Number of words of length {:2} = {}", key, wordbase[key].words.len());
    }

    wordbase
}

fn calc_reachable_words_for_table(wt: &WordTable) -> Vec<AnchoredWords> {
    let mut all_anchored_words = Vec::new();
    for w1 in &wt.words {
        let mut anchored_words = AnchoredWords::new(w1.clone());
        // Followed by all the words that are one letter different.
        for w2 in &wt.words {
            if one_letter_different(w1, w2) {
                anchored_words.add_reachable_word(w2.clone());
            }
        }

        all_anchored_words.push(anchored_words);
    }

    all_anchored_words
}

fn write_difference_file(word_length: usize, anchored_words: &Vec<AnchoredWords>) {
    let rw_filename = format!("{}/one_letter_different_{:02}.txt", dictionary::DICT_OUT, word_length);
    println!("Writing {}", rw_filename);
    let rw_file = fs::File::create(rw_filename).unwrap();
    let mut writer = io::BufWriter::new(rw_file);
    for v in anchored_words {
        // Words which have no other reachable words are not interesting.
        if v.reachable_words.len() == 0 { continue };

        write!(writer, "{} ", v.anchor).unwrap();
        for w in &v.reachable_words {
            write!(writer, "{} ", w).unwrap();
        }
        write!(writer, "\n").unwrap();
    }
}

fn one_letter_different(w1: &str, w2: &str) -> bool {
    assert_eq!(w1.len(), w2.len());

    let mut num_diffs = 0;
    for (a, b) in w1.chars().zip(w2.chars()) {
        if a != b {
            num_diffs += 1;
        }
        if num_diffs == 2 {
            return false;
        }
    }

    return num_diffs == 1
}
