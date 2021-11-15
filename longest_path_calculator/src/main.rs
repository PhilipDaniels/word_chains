use std::{collections::{HashMap, HashSet}, fs::File, hash::Hash, io, num, path::PathBuf};
use std::io::Write;

use completed_words::{CompletedWords, create_chain_directories, get_completed_words};
use graph::{Graph, RelativeDirectories};
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    vec,
};
use structopt::StructOpt;

mod completed_words;

#[derive(Debug, StructOpt)]
struct CommandLineOptions {
    #[structopt(name = "DICTIONARY_DIR", parse(from_os_str))]
    dictionary_directory: PathBuf,
}

fn main() {
    let options = CommandLineOptions::from_args();
    let dirs = RelativeDirectories::new(&options.dictionary_directory);

    if !dirs.output_directory().exists() {
        eprintln!(
            "Output directory {:?} does not exist",
            dirs.output_directory()
        );
        std::process::exit(1);
    }

    let mut graphs = load_graphs(&dirs);

    // Determine completed words by scanning output\chainsNN files.
    let word_lengths: Vec<_> = graphs.iter().map(|g| g.word_length()).collect();
    create_chain_directories(&dirs, &word_lengths);
    let completed_words = get_completed_words(&dirs, &word_lengths);
    print_completion_status(&graphs, &completed_words);

    // Sort graphs by increasing size so that we handle the smallest ones first.
    // Then it looks like we are making progress...
    graphs.sort_unstable_by(|a, b| a.size().cmp(&b.size()));

    // Calculate remaining words in the above order.
    for graph in &graphs {
        let completed_already = completed_words.completed_words_of_length(graph.word_length());
        calculate_longest_path(&dirs, graph, completed_already);
    }
}

fn load_graphs(dirs: &RelativeDirectories) -> Vec<Graph> {
    (1..30)
        .into_par_iter()
        .filter_map(|word_length| {
            let filename = dirs.largest_component_adjacency_file(word_length);
            let g_result = Graph::load_from_adjacency_file(&filename);
            if g_result.is_ok() {
                println!(
                    "Loaded graph of size {} from {:?}",
                    g_result.as_ref().unwrap().size(),
                    filename
                );
            }
            g_result.ok()
        })
        .collect()
}

fn print_completion_status(graphs: &[Graph], completed_words: &CompletedWords) {
    for graph in graphs {
        let word_length = graph.word_length();
        let num_complete = completed_words.num_complete(word_length);
        let percent = if num_complete == 0 {
            0.0
        } else {
            100.0 * num_complete as f64 / graph.size() as f64
        };

        println!("Graph {} is {:.2}% complete", word_length, percent);
    }
}

fn calculate_longest_path(dirs: &RelativeDirectories, graph: &Graph, completed_already: &[String]) {
    let all_words: HashSet<String> = graph.vertices.iter().map(|v| v.word.clone()).collect();
    let completed_already: HashSet<String> = completed_already.iter().cloned().collect();
    let words_still_to_do: Vec<_> = all_words.difference(&completed_already).collect();
    if words_still_to_do.len() == 0 {
        return;
    }

    println!(
        "There are {} words still to compute for the graph of word length {}",
        words_still_to_do.len(),
        graph.word_length()
    );

    words_still_to_do.into_par_iter().for_each(|word| {
        let path = calculate_longest_path_for_word(dirs, graph, word);
        write_path_output_file(dirs, &path);
    });

    // ALL DONE BY HERE!
    // scan files in directory chainsNN
    // find the file with the longest number of words (split by space)
    // write file chainsNN\00_longest_path.txt

}

fn calculate_longest_path_for_word(
    dirs: &RelativeDirectories,
    graph: &Graph,
    word: &String,
) -> Vec<String> {
    vec![]
}

/// Writes the output file 'output\chainsNN\{word}.txt'.
fn write_path_output_file(dirs: &RelativeDirectories, path: &Vec<String>) {
    assert!(path.len() > 1);
    let anchor_word = &path[0];
    let mut filename = dirs.chains_directory(anchor_word.len());
    std::fs::create_dir_all(&filename).unwrap();
    filename.push(format!("{}.txt", anchor_word));

    let out_file = File::create(&filename).expect("Unable to create chain file");
    let mut writer = io::BufWriter::new(out_file);
    let words = path.join(" ");
    writeln!(writer, "{}", words).unwrap();

    println!("Wrote a chain of length {} to {:?}", path.len(),  filename);
}
