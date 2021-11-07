use graph::{Graph, get_graph_stats};

fn main() {
    // Loading the graphs and calculating components is very fast, there
    // is no reason not to do it for all of them.

    let graphs: Vec<_> = (1..30)
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
