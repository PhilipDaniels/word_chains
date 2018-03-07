use std::fs;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::io::prelude::*;

use ::dictionary;

type WordBase = HashMap<usize, Vec<String>>;

pub fn calculate_reachable_words() {
    let wordbase = get_words_by_length();

    let keys = sorted_keys(&wordbase);
    for key in keys {
        let words = &wordbase[&key];
        let word_length = words.len();
        if words.len() <= 2 { continue };

        let rw_filename = format!("{}/reachable_words_{:2}.txt", dictionary::DICT_OUT, words.len());

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
            let len = word.len();
            let v = wordbase.entry(len).or_insert(Vec::new());
            v.push(word);
        }

        println!("Finished reading {}", dictionary::CORPUS);

        let keys = sorted_keys(&wordbase);

        for key in keys {
            println!("Number of words of length {:2} = {}", key, &wordbase[&key].len());
        }
    }

    wordbase
}

fn sorted_keys(wordbase: &WordBase) -> Vec<usize> {
    let mut keys : Vec<usize> = wordbase.keys().map(|&k| k).collect();
    keys.sort();
    keys
}
