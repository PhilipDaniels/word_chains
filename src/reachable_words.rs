use std::fs;
//use std::collections::HashMap;
use std::io::{self, BufRead};
//use std::io::prelude::*;

use ::dictionary;
use ::word_base::WordBase;
//use ::word_table::WordTable;

pub fn calculate_reachable_words() {
    let wordbase = get_words_by_length();

    let keys = wordbase.sorted_keys();
    for key in keys {
        let word_table = &wordbase[key];
        if word_table.words.len() <= 2 { continue };
        let rw_filename = format!("{}/reachable_words_{:02}.txt", dictionary::DICT_OUT, word_table.word_length());
        println!("Writing {}", rw_filename);
    }
}

fn get_words_by_length() -> WordBase {
    let f = fs::File::open(dictionary::CORPUS).unwrap();
    let rdr = io::BufReader::new(f);

    let mut wordbase = WordBase::new();

    {
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
    }

    wordbase
}


