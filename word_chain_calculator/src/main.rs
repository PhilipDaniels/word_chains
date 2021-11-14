use std::ops::RangeInclusive;
use rayon::prelude::*;

use graph::{Graph, get_graph_stats};

fn main() {
    // Loading the graphs and calculating components is very fast, there
    // is no reason not to do it for all of them. But it's handy to be
    // able to specify one, for debugging purposes.

    let word_lengths = match std::env::args().nth(1) {
        Some(s) => {
            let n = s.parse::<usize>().unwrap();
            RangeInclusive::new(n, n)
        },
        None => RangeInclusive::new(1, 30)
    };

    let mut graphs: Vec<_> = word_lengths
        .into_par_iter()
        .filter_map(|word_length| {
            let filename = format!("../dictionaries_out/one_letter_different_{:02}.txt", word_length);

            Graph::load_from_difference_file(&filename)
                .map(|graph| {
                    let stats = get_graph_stats(&graph);
                    (graph, stats)
                })
                .ok()
        })
        .collect();

    graphs.sort_unstable_by(|a, b| a.0.word_length().cmp(&b.0.word_length()));
    for gs in &graphs {
        println!("{:?}", gs.1);
    }
}

/*
TxMax is the maximum number of reachable words for a particular word in the graph
TxMaxWord is that word
TxMaxWordReachables is all the words that can be reached from it.graph

Ability to extract just the largest component into a separate graph
 */
