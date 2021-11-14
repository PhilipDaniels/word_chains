use rayon::prelude::*;
use std::ops::RangeInclusive;

use graph::{calculate_graph_stats, Graph, WordLengthStatistics};

fn main() {
    // Loading the graphs and calculating components is reasonably fast,
    // there is no reason not to do it for all of them. But it's handy to be
    // able to specify one, for debugging purposes.

    let word_lengths = match std::env::args().nth(1) {
        Some(s) => {
            let n = s.parse::<usize>().unwrap();
            RangeInclusive::new(n, n)
        }
        None => RangeInclusive::new(1, 30),
    };

    let mut graphs: Vec<_> = word_lengths
        .into_par_iter()
        .filter_map(|word_length| {
            let filename = format!(
                "../dictionaries_out/one_letter_different_{:02}.txt",
                word_length
            );

            Graph::load_from_difference_file(&filename)
                .map(|graph| {
                    let stats = calculate_graph_stats(&graph);
                    println!("Loaded graph for word length of {}", stats.word_length);
                    (graph, stats)
                })
                .ok()
        })
        .collect();

    graphs.sort_unstable_by(|a, b| a.0.word_length().cmp(&b.0.word_length()));
    write_graph_stats(&graphs);
}

fn write_graph_stats(graphs: &[(Graph, WordLengthStatistics)]) {
    let mut writer = csv::Writer::from_path("./../dictionaries_out/word_graph_stats.csv").unwrap();

    writer
        .write_record(&[
            "Len",
            "WordCount",
            "ComponentCount",
            "1-Components",
            "2-Components",
            "3-Components",
            "Top5-Components",
            "LargestComponentSize",
            "LargestComponentLeafCount",
            "LargestComponentUpperBound",
            "LargestComponentPercent",
            "MaxAdjacentsCount",
            "MaxAdjacentsWord",
            "MaxAdjacentsList"
        ])
        .unwrap();

    for (_g, stats) in graphs {
        writer
            .serialize((
                stats.word_length,
                stats.total_word_count,
                stats.num_components,
                stats.num_one_components,
                stats.num_two_components,
                stats.num_three_components,
                stats
                    .largest_five_component_counts
                    .iter()
                    .fold("".to_string(), |mut acc, n| {
                        if acc.is_empty() {
                            acc += &n.to_string();
                        } else {
                            acc += ",";
                            acc +=&n.to_string();
                        }

                        acc
                    }),
                stats.largest_component_word_count(),
                stats.largest_component_leaf_count,
                stats.largest_component_upper_bound(),
                format!("{:.2}", stats.largest_component_percent_of_total()),
                stats.max_adjacents_count,
                &stats.max_adjacents_word,
                stats.max_adjacencts_list.join(",")
            ))
            .unwrap();
    }
}

/*
Ability to extract just the largest component into a separate graph
 */
