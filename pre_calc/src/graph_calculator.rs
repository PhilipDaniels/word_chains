use graph::{calculate_graph_stats, Graph, WordLengthStatistics};
use rayon::prelude::*;
use std::io::Write;
use std::path::Path;
use std::{fs, io, ops::RangeInclusive};

use crate::CommandLineOptions;

pub(crate) fn calculate_initial_graphs(options: &CommandLineOptions) {
    // Loading the graphs and calculating components is reasonably fast,
    // there is no reason not to do it for all of them. But it's handy to be
    // able to specify one, for debugging purposes.
    // let word_lengths = match std::env::args().nth(1) {
    //     Some(s) => {
    //         let n = s.parse::<usize>().unwrap();
    //         RangeInclusive::new(n, n)
    //     }
    //     None => RangeInclusive::new(1, 30),
    // };

    let word_lengths = RangeInclusive::new(1, 30);

    let (mut graphs, stats): (Vec<_>, Vec<_>) = word_lengths
        .into_par_iter()
        .filter_map(|word_length| {
            let filename = options.all_adjacency_file(word_length);

            Graph::load_from_adjacency_file(&filename)
                .map(|graph| {
                    println!(
                        "Loaded graph for word length of {} from {:?}",
                        word_length, filename
                    );

                    let stats = calculate_graph_stats(&graph);
                    (graph, stats)
                })
                .ok()
        })
        .unzip();

    graphs.sort_unstable_by(|a, b| a.word_length().cmp(&b.word_length()));
    write_word_stats(&options.word_stats_file(), &stats);
    write_largest_components_to_file(options, &graphs);
}

fn write_word_stats(stats_file: &Path, stats: &[WordLengthStatistics]) {
    let mut writer = csv::Writer::from_path(stats_file).unwrap();

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
            "MaxAdjacentsList",
        ])
        .unwrap();

    for stat in stats {
        writer
            .serialize((
                stat.word_length,
                stat.total_word_count,
                stat.num_components,
                stat.num_one_components,
                stat.num_two_components,
                stat.num_three_components,
                stat.largest_five_component_counts
                    .iter()
                    .fold("".to_string(), |mut acc, n| {
                        if acc.is_empty() {
                            acc += &n.to_string();
                        } else {
                            acc += ",";
                            acc += &n.to_string();
                        }

                        acc
                    }),
                stat.largest_component_word_count(),
                stat.largest_component_leaf_count,
                stat.largest_component_upper_bound(),
                format!("{:.2}", stat.largest_component_percent_of_total()),
                stat.max_adjacents_count,
                &stat.max_adjacents_word,
                stat.max_adjacencts_list.join(","),
            ))
            .unwrap();
    }
}

/// Extract the largest component from each graph and write it as its
/// own adjacency list file, to speed up and simplify for further processing.
fn write_largest_components_to_file(options: &CommandLineOptions, graphs: &[Graph]) {
    for graph in graphs {
        let components = graph.components();
        let comp = components
            .get(0)
            .expect("At least one component should exist");

        let filename = options.largest_component_adjacency_file(graph.word_length());
        println!("Writing {:?}", filename);
        let rw_file = fs::File::create(filename).unwrap();
        let mut writer = io::BufWriter::new(rw_file);

        for v in graph.vertices.iter().filter(|v| v.component == comp.number) {
            write!(writer, "{} ", v.word).unwrap();

            for word_index in &v.adjacency_list {
                let v2 = &graph.vertices[*word_index];
                write!(writer, "{} ", v2.word).unwrap();
            }

            writeln!(writer).unwrap();
        }
    }
}
