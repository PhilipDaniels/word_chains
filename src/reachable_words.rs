use std::fs;
//use std::collections::HashMap;
use std::io::{self, BufRead, BufWriter};
use std::io::prelude::*;

use ::dictionary;
use ::word_base::WordBase;
use ::word_table::WordTable;

pub fn calculate_reachable_words() {
    let wordbase = get_words_by_length();

    let keys = wordbase.sorted_keys();
    for key in keys {
        let word_table = &wordbase[key];
        if word_table.words.len() <= 2 { continue };
        calc_reachable_words_for_table(&word_table);
    }
}

fn get_words_by_length() -> WordBase {
    let f = fs::File::open(dictionary::CORPUS).unwrap();
    let rdr = io::BufReader::new(f);

    let mut wordbase = WordBase::new();

    println!("Start reading {}", dictionary::CORPUS);

    for word in rdr.lines() {
        let word = word.unwrap();
        wordbase.add_word(word);
    }

    println!("Finished reading {}", dictionary::CORPUS);

    let keys = wordbase.sorted_keys();

    for key in keys {
        println!("Number of words of length {:2} = {}", key, wordbase[key].words.len());
    }

    wordbase
}

fn calc_reachable_words_for_table(wt: &WordTable) {
    let mut all_rwords = Vec::new();
    for w1 in &wt.words {
        let mut rwords = Vec::new();
        rwords.push(w1);

        for w2 in &wt.words {
            if one_letter_different(w1, w2) {
                rwords.push(w2);
            }
        }

        all_rwords.push(rwords);
    }

    let rw_filename = format!("{}/reachable_words_{:02}.txt", dictionary::DICT_OUT, wt.word_length());
    println!("Writing {}", rw_filename);
    let rw_file = fs::File::create(rw_filename).unwrap();
    let mut writer = io::BufWriter::new(rw_file);
    for v in &all_rwords {
        // Words which have no other reachable words are not interesting.
        if v.len() < 2 { continue };

        for w in v.iter() {
            write!(writer, "{} ", w);
        }
        write!(writer, "\n");
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