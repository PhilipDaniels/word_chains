use std::ops::RangeInclusive;

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

    let _graphs: Vec<_> = word_lengths
        .filter_map(|word_length| {
            let filename = format!("../dictionaries_out/one_letter_different_{:02}.txt", word_length);

            Graph::load_from_difference_file(&filename)
                .map(|graph| {
                    let stats = get_graph_stats(&graph);
                    println!("{:?}", stats);
                    (graph, stats)
                })
                .ok()
        })
        .collect();
}
