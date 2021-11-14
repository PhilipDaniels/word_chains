use rayon::prelude::*;
use std::io::Write;
use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
    ops::{AddAssign, Index, IndexMut},
    path::Path,
};

use crate::CommandLineOptions;

/// Reads in the entire word corpus and for each word, calculates its adjacency list,
/// that is, all the words that can be formed by changing just a single character
/// in the original word.
pub(crate) fn calculate_corpus_adjacency_lists(options: &CommandLineOptions) {
    println!(
        "Calculating word adjacency lists based on {:?}",
        options.corpus_file()
    );
    let corpus = read_corpus_file(&options.corpus_file());

    println!("Finished reading {:?}", &options.corpus_file());
    let keys = corpus.sorted_keys();
    for key in &keys {
        println!(
            "Number of words of length {:2} = {}",
            key,
            corpus[*key].len()
        );
    }

    keys.par_iter().for_each(|&key| {
        let words = &corpus[key];
        if words.len() <= 2 {
            return;
        };

        let adjacency_lists = calc_adjacency_lists(words);
        write_adjacency_list_file(options, &adjacency_lists);
    });
}

fn read_corpus_file(corpus_file: &Path) -> Corpus {
    let f = fs::File::open(corpus_file).unwrap();
    let rdr = io::BufReader::new(f);

    let mut corpus = Corpus::new();

    for word in rdr.lines() {
        let word = word.unwrap();
        corpus += word;
    }

    corpus
}

fn calc_adjacency_lists(words: &[String]) -> Vec<WordAdjacencyList> {
    words
        .par_iter()
        .map(|w1| {
            let mut adjaceny_list = WordAdjacencyList::new(w1.clone());

            // Find all the words that are one letter different.
            for w2 in words {
                if one_letter_different(w1, w2) {
                    adjaceny_list.add_adjacent_word(w2.clone());
                }
            }

            adjaceny_list
        })
        .collect()
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

/// Writes one adjacency list file for a particular word length.
fn write_adjacency_list_file(options: &CommandLineOptions, adjacency_lists: &[WordAdjacencyList]) {
    if adjacency_lists.is_empty()
        || adjacency_lists
            .iter()
            .all(|aw| aw.adjacent_words.is_empty())
    {
        return;
    }

    let word_length = adjacency_lists[0].anchor.len();
    let filename = options.all_adjacency_file(word_length);
    println!("Writing {:?}", filename);
    let rw_file = fs::File::create(filename).unwrap();
    let mut writer = io::BufWriter::new(rw_file);

    for v in adjacency_lists {
        // Words which have no other reachable words are not interesting.
        // ...except for generating stats later on.
        //if v.reachable_words.is_empty() {
        //    continue;
        //};

        write!(writer, "{} ", v.anchor).unwrap();
        let words = v.adjacent_words.join(" ");
        writeln!(writer, "{}", words).unwrap();
    }
}

struct WordAdjacencyList {
    anchor: String,
    adjacent_words: Vec<String>,
}

impl WordAdjacencyList {
    fn new(anchor: String) -> Self {
        WordAdjacencyList {
            anchor,
            adjacent_words: Vec::new(),
        }
    }

    fn add_adjacent_word(&mut self, word: String) {
        self.adjacent_words.push(word);
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

/// Represents all the words we are interested in, organized
/// in a HashMap by their length.
pub struct Corpus {
    pub words: HashMap<usize, Vec<String>>,
}

impl Corpus {
    pub fn new() -> Self {
        Corpus {
            words: HashMap::new(),
        }
    }

    pub fn sorted_keys(&self) -> Vec<usize> {
        let mut keys: Vec<_> = self.words.keys().copied().collect();
        keys.sort_unstable();
        keys
    }
}

impl Index<usize> for Corpus {
    type Output = [String];

    fn index(&self, word_length: usize) -> &Self::Output {
        &self.words[&word_length]
    }
}

impl IndexMut<usize> for Corpus {
    fn index_mut(&mut self, word_length: usize) -> &mut Self::Output {
        self.words.get_mut(&word_length).unwrap()
    }
}

impl AddAssign<String> for Corpus {
    /// Adds a new word to the corpus in the appropriate slot.
    fn add_assign(&mut self, word: String) {
        let word_vec = self.words.entry(word.len()).or_insert_with(|| Vec::new());
        word_vec.push(word);
    }
}
