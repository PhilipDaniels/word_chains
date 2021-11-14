use std::path::PathBuf;

use adjacency_calculator::calculate_corpus_adjacency_lists;
use dictionary_merger::merge_dictionaries;
use graph_calculator::calculate_initial_graphs;
use structopt::StructOpt;

mod adjacency_calculator;
mod dictionary_merger;
mod graph_calculator;

#[derive(Debug, StructOpt)]
struct CommandLineOptions {
    #[structopt(short = "1", long)]
    merge_dictionaries: bool,

    #[structopt(short = "2", long)]
    calc_adjacency_lists: bool,

    #[structopt(short = "3", long)]
    calc_graphs: bool,

    #[structopt(name = "DICTIONARY_DIR", parse(from_os_str))]
    dictionary_directory: PathBuf,
}

fn main() {
    let options = CommandLineOptions::from_args();

    if !options.dictionary_directory.exists() {
        eprintln!(
            "Dictionary directory {:?} does not exist",
            options.dictionary_directory
        );
    }

    if options.merge_dictionaries {
        merge_dictionaries(&options.dictionary_directory, &options.corpus_file());
    }

    if options.calc_adjacency_lists {
        calculate_corpus_adjacency_lists(&options);
    }

    if options.calc_graphs {
        calculate_initial_graphs(&options);
    }
}

impl CommandLineOptions {
    /// Returns the directory into which output results
    /// are to be stored.
    pub fn output_directory(&self) -> PathBuf {
        let mut pb = self.dictionary_directory.parent().unwrap().to_path_buf();
        pb.push("output");
        pb
    }

    /// Returns the filename of the corpus file.
    pub fn corpus_file(&self) -> PathBuf {
        let mut pb = self.output_directory();
        pb.push("corpus.txt");
        pb
    }

    /// Returns the name of the 'all adjacenies' file for a particular word length.
    pub fn all_adjacency_file(&self, word_length: usize) -> PathBuf {
        let mut pb = self.output_directory();
        pb.push(format!("all_adjacency_lists_{:02}.txt", word_length));
        pb
    }

    /// Returns the name of the file which will be used to hold the adjacency
    /// lists which construct the largest component.
    pub fn largest_component_adjacency_file(&self, word_length: usize) -> PathBuf {
        let mut pb = self.output_directory();
        pb.push(format!("largest_component_adjacency_lists_{:02}.txt", word_length));
        pb
    }

    /// Returns the name of the file which will hold word length statistics
    /// computed from the graph.
    pub fn word_stats_file(&self) -> PathBuf {
        let mut pb = self.output_directory();
        pb.push("word_stats.csv");
        pb
    }
}
