use std::fs;
use std::io::prelude::*;
use std::io::{self, BufRead};

mod corpus;
mod graph;

use corpus::{Corpus, WordSet};

const DICT_OUT_DIR: &str = "./../dictionaries_out";
const CORPUS_FILE: &str = "./../dictionaries_out/corpus.txt";

fn main() {
    let corpus = get_words_by_length();

    let keys = corpus.sorted_keys();
    // This lot can be done in parallel.
    for key in keys {
        let word_table = &corpus[key];
        if word_table.words.len() <= 2 {
            continue;
        };
        let rwords_for_table = calc_reachable_words_for_table(word_table);
        // This is the initial difference file, includes 2-islands.
        write_difference_file(word_table.word_length(), &rwords_for_table);
        //println!("Calculating graph");
        //let _g = make_graph(&rwords_for_table);
        //println!("Calculating graph finished");
    }
}

/// Read in the entire word CORPUS and form it into a WordMap.
fn get_words_by_length() -> Corpus {
    println!("Start reading {}", CORPUS_FILE);

    let f = fs::File::open(CORPUS_FILE).unwrap();
    let rdr = io::BufReader::new(f);

    let mut corpus = Corpus::new();

    for word in rdr.lines() {
        let word = word.unwrap();
        corpus += word;
    }

    println!("Finished reading {}", CORPUS_FILE);

    let keys = corpus.sorted_keys();

    for key in keys {
        println!(
            "Number of words of length {:2} = {}",
            key,
            corpus[key].words.len()
        );
    }

    corpus
}

fn calc_reachable_words_for_table(wt: &WordSet) -> Vec<AnchoredWords> {
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

    num_diffs == 1
}

fn write_difference_file(word_length: usize, anchored_words: &[AnchoredWords]) {
    let rw_filename = format!(
        "{}/one_letter_different_{:02}.txt",
        DICT_OUT_DIR,
        word_length
    );

    println!("Writing {}", rw_filename);
    let rw_file = fs::File::create(rw_filename).unwrap();
    let mut writer = io::BufWriter::new(rw_file);

    for v in anchored_words {
        // Words which have no other reachable words are not interesting.
        if v.reachable_words.is_empty() {
            continue;
        };

        write!(writer, "{} ", v.anchor).unwrap();
        for w in &v.reachable_words {
            write!(writer, "{} ", w).unwrap();
        }
        writeln!(writer).unwrap();
    }
}












struct AnchoredWords {
    anchor: String,
    reachable_words: Vec<String>,
}

impl AnchoredWords {
    fn new(anchor: String) -> Self {
        AnchoredWords {
            anchor,
            reachable_words: Vec::new(),
        }
    }

    fn add_reachable_word(&mut self, word: String) {
        self.reachable_words.push(word);
    }
}




/*
fn make_graph(anchored_words: &[AnchoredWords]) -> Graph {
    let mut g = Graph::new();

    // First load all the anchor words so the graph can calculate their indexes.
    // Ignore anchor words with no reachable words, they are not interesting.
    let interesting_words: Vec<&AnchoredWords> = anchored_words
        .iter()
        .filter(|aw| !aw.reachable_words.is_empty())
        .collect();

    for aw in &interesting_words {
        g.add_anchor_word(&aw.anchor);
    }

    // Then we can add all the reachable words.
    for aw in &interesting_words {
        for rw in &aw.reachable_words {
            g.add_reachable_word(&aw.anchor, rw);
        }
    }

    g.calculate_components();
    g
}
*/




/*
void CalculateReachableWordsUsingPrefix()
{
    // First calculate all the prefixes. This is a mapping of the
    // form 'pre -> Prefix("pre")'. As we process each word we add
    // it to the reachable words collection of the relevant prefix.
    Prefix[string] prefixes;
    foreach (word; _words)
    {
        auto prefix = word.GetPrefix();
        auto p = prefix in prefixes;
        if (p)
        {
            (*p).AddReachableWord(word);
        }
        else
        {
            auto p2 = new Prefix(prefix);
            p2.AddReachableWord(word);
            prefixes[prefix] = p2;
        }
    }
    writef("Word Len = %s, %s prefixes calculated, ",
           _word_length, prefixes.length);
    fflush(std.c.stdio.stdout);

    // Determine which prefixes can reach other prefixes.
    foreach (p1; prefixes)
    {
        foreach (p2; prefixes)
        {
            if (OneLetterDifferent(p1.GetPrefix(), p2.GetPrefix()))
                p1.AddReachablePrefix(p2);
        }
    }
    write("cross linked, ");
    fflush(std.c.stdio.stdout);

    // Now calculate the reachable words.
    foreach (anchor_word; _words)
    {
        auto prefix = prefixes[anchor_word.GetPrefix()];
        AddWordsFromPrefix(anchor_word, prefix);
    }
    writeln("and reachable words found.");
}

void AddWordsFromPrefix(Word anchor, Prefix prefix)
{
    // Add all the words directly reachable from this prefix.
    foreach (rword, dummy; prefix.ReachableWords())
    {
        if (OneLetterDifferent(anchor.GetWord(), rword.GetWord()))
            anchor.AddReachableWord(rword);
    }

    // Add all the words directly reachable from the prefixes
    // we are iterating over.
    foreach (rp, dummy; prefix.ReachablePrefixes())
    {
        foreach (rword, dummy2; rp.ReachableWords())
        {
            if (OneLetterDifferent(anchor.GetWord(), rword.GetWord()))
                anchor.AddReachableWord(rword);
        }
    }
}
*/
