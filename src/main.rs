use std::fs;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::io::prelude::*;

fn main() {
    create_merged_dictionary();
}

/// Reads all the available input dictionaries, filters the words for basic acceptability, and
/// then creates a single merged dictionary called "corpus.txt" in the current folder.
fn create_merged_dictionary() {
    let mut words = HashSet::new();
    let input_dir = "./dictionaries";
    let paths = fs::read_dir(input_dir).expect("Could not locate dictionaries folder");

    for path in paths {
        let path = path.unwrap().path();
        println!("Reading words from {:?}", path);

        let num_words_at_start = words.len();
        let file = fs::File::open(path).expect("no such file");
        let mut rdr = io::BufReader::new(file);

        let mut num_words_read_from_file = 0;
        for v in rdr.split(b'\n') {
            match v {
                Ok(bytes) => {
                    num_words_read_from_file += 1;
                    match String::from_utf8(bytes) {
                        Ok(word) => { if let Some(w) = clean_word(word) { words.insert(w); }},
                        Err(_) => {}
                    }
                },
                Err(_) => {}
            }
        }

        let num_words_added = words.len() - num_words_at_start;
        println!("    Read {} words and added {}", num_words_read_from_file, num_words_added);
    }

    println!("Total word count = {}", words.len());

    let out_file = fs::File::create("./dictionaries_out/corpus.txt").expect("Can create corpus.txt");
    let mut writer = io::BufWriter::new(out_file);
    let mut words : Vec<String> = words.into_iter().collect();
    words.sort();
    for w in words {
        write!(writer, "{}\n", w).unwrap();
    }

    println!("Wrote corpus.txt");
}

fn clean_word(w: String) -> Option<String> {
    if w.len() <= 2 {
        return None;
    }

    let w = w.to_lowercase().trim().to_string();
    if w.chars().all(|c| c.is_ascii() && c.is_ascii_lowercase()) {
        Some(w)
    } else {
        None
    }
}