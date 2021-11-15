use std::path::PathBuf;

use adjacency_calculator::calculate_corpus_adjacency_lists;
use dictionary_merger::merge_dictionaries;
use graph::RelativeDirectories;
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
    let dirs = RelativeDirectories::new(&options.dictionary_directory);

    if !dirs.dictionary_directory().exists() {
        eprintln!(
            "Dictionary directory {:?} does not exist",
            dirs.dictionary_directory()
        );
        std::process::exit(1);
    }

    if options.merge_dictionaries {
        merge_dictionaries(&dirs.dictionary_directory(), &dirs.corpus_file());
    }

    if options.calc_adjacency_lists {
        calculate_corpus_adjacency_lists(&dirs);
    }

    if options.calc_graphs {
        calculate_initial_graphs(&dirs);
    }
}
