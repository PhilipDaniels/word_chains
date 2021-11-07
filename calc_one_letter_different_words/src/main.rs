use std::fs;
use std::io::prelude::*;
use std::io::{self, BufRead};

mod corpus;

use corpus::Corpus;
use rayon::prelude::*;

const DICT_OUT_DIR: &str = "./../dictionaries_out";
const CORPUS_FILE: &str = "./../dictionaries_out/corpus.txt";

fn main() {
    let corpus = read_corpus_file();
    let keys = corpus.sorted_keys();

    keys.par_iter()
        .for_each(|&key| {
            let words = &corpus[key];
            if words.len() <= 2 {
                return;
            };

            let reachable_words = calc_reachable_words(words);
            write_difference_file(&reachable_words);
        });
}

/// Read in the entire word CORPUS and form it into a WordMap.
fn read_corpus_file() -> Corpus {
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
            corpus[key].len()
        );
    }

    corpus
}

fn calc_reachable_words(words: &[String]) -> Vec<AnchoredWords> {
    words.par_iter()
        .map(|w1| {
            let mut anchored_words = AnchoredWords::new(w1.clone());

            // Find all the words that are one letter different.
            for w2 in words {
                if one_letter_different(w1, w2) {
                    anchored_words.add_reachable_word(w2.clone());
                }
            }

            anchored_words
        }).collect()
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

fn write_difference_file(anchored_words: &[AnchoredWords]) {
    if anchored_words.is_empty()
        || anchored_words
            .iter()
            .all(|aw| aw.reachable_words.is_empty())
    {
        return;
    }

    let word_length = anchored_words[0].anchor.len();

    let filename = format!(
        "{}/one_letter_different_{:02}.txt",
        DICT_OUT_DIR, word_length
    );

    println!("Writing {}", filename);
    let rw_file = fs::File::create(filename).unwrap();
    let mut writer = io::BufWriter::new(rw_file);

    for v in anchored_words {
        // Words which have no other reachable words are not interesting.
        if v.reachable_words.is_empty() {
            continue;
        };

        write!(writer, "{} ", v.anchor).unwrap();
        let words = v.reachable_words.join(" ");
        writeln!(writer, "{}", words).unwrap();
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
