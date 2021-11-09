use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
use std::{collections::HashMap, fs::File, path::Path};

#[derive(Debug)]
pub struct Vertex {
    word: String,
    adjacency_list: Vec<usize>,
    component: usize,
}

impl From<String> for Vertex {
    fn from(word: String) -> Self {
        Self {
            word,
            adjacency_list: Vec::new(),
            component: usize::MAX,
        }
    }
}

/// Represents a graph of words of length N. This is really a forest, because
/// there may be (in fact, probably are) multiple components within the graph.
#[derive(Debug)]
pub struct Graph {
    vertices: Vec<Vertex>,
    // Provide a fast way of looking up the index of a word.
    word_to_index: HashMap<String, usize>,
}

impl Graph {
    /// Reads in the specified difference file (e.g. "one_letter_different_05.txt")
    /// and returns a graph with all its vertices correctly linked and its
    /// components calculated.
    pub fn load_from_difference_file<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let f = File::open(filename)?;
        let rdr = BufReader::new(f);
        let lines = rdr.lines().collect::<io::Result<Vec<String>>>()?;

        // Each line consists of 2 or more words.
        // The first word is the 'anchor', and the remaining words are the 'reachable words':
        // the anchor can be transformed into each of the reachable words by changing
        // one letter in the anchor, thus each line represents a 1-step transformation.
        //
        // We can use this to build a graph and then calculate its components. A component
        // is a set of words which can all be reached by N-step transformations.
        let mut graph = Graph {
            vertices: Vec::new(),
            word_to_index: HashMap::new(),
        };

        // Create a vertex for each anchor word. We have to do this first
        // so that they are all created before we calculate the adjacency list
        // (we need the index into the vertices vector to do that).
        // When this is complete there will be a vertex in the graph for
        // every word in the file (which really means every anchor word - it
        // should be apparent that there are no reachable words which are not
        // also anchor words, because for any word pair "A B" then "B A"
        // will also appear in the file).
        for line in &lines {
            let anchor_word = line.split(' ').next().unwrap();
            graph.add_vertex(anchor_word);
        }

        for line in lines {
            let mut words_in_line = line.split(' ');
            let anchor_word = words_in_line.next().unwrap();
            let anchor_word_index = graph.get_index_for_word(anchor_word);

            graph.vertices[anchor_word_index].adjacency_list =
                words_in_line.map(|w| graph.get_index_for_word(w)).collect();
        }

        graph.calculate_components();
        Ok(graph)
    }

    fn add_vertex<S: Into<String>>(&mut self, word: S) {
        let word = word.into();

        let vertex = word.clone().into();
        self.vertices.push(vertex);

        self.word_to_index.insert(word, self.vertices.len() - 1);
    }

    // TODO: Implement Index<str>.
    fn get_index_for_word(&self, word: &str) -> usize {
        self.word_to_index[word]
    }

    /// Calculates the components of the graph. All the vertices in the graph are examined,
    /// and all those that are reachable from each other (in 1 or more steps) are assigned
    /// the same component number.
    fn calculate_components(&mut self) {
        let mut next_component_number = 0;

        while let Some(idx) = self.vertices.iter().position(|v| v.component == usize::MAX) {
            // First find all the vertices in this component and record their indexes in `seen`.
            // This is a read-only operation.
            let mut seen = HashSet::new();
            self.dfs(&mut seen, idx);

            // Then set the component on all those nodes.
            for i in 0..self.vertices.len() {
                if seen.contains(&i) {
                    self.vertices[i].component = next_component_number;
                }
            }

            next_component_number += 1;
        }
    }

    fn dfs(&self, seen: &mut HashSet<usize>, idx: usize) {
        seen.insert(idx);
        let v = &self.vertices[idx];

        for i2 in &v.adjacency_list {
            if seen.contains(i2) {
                continue;
            }

            let v2 = &self.vertices[idx];
            if v2.component == usize::MAX {
                self.dfs(seen, *i2);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct WordLengthStatistics {
    pub word_length: usize,
    pub total_word_count: usize,
    pub num_components: usize,
    pub num_one_islands: usize,
    pub num_two_islands: usize,
    pub num_three_islands: usize,
    pub largest_five_component_counts: Vec<usize>,
    pub lc_leaf_count: usize,
}

impl WordLengthStatistics {
    pub fn largest_component_word_count(&self) -> usize {
        self.largest_five_component_counts[0]
    }

    pub fn largest_component_percent_of_total(&self) -> f64 {
        self.largest_component_word_count() as f64 / self.total_word_count as f64
    }

    pub fn largest_component_upper_bound(&self) -> usize {
        self.largest_component_word_count() - self.lc_leaf_count + 2
    }
}

/// Calculates various interesting statistics for a word graph.
pub fn get_graph_stats(graph: &Graph) -> WordLengthStatistics {
    let mut stats = WordLengthStatistics {
        word_length: graph.vertices[0].word.len(),
        total_word_count: graph.vertices.len(),
        ..Default::default()
    };
    
    let components = components(graph);
   
    stats.num_components = components.len();

    stats.num_one_islands = components
        .iter()
        .filter(|c| c.num_vertices == 1)
        .count();
    
    stats.num_two_islands = components
        .iter()
        .filter(|c| c.num_vertices == 2)
        .count();
    
    stats.num_three_islands = components
        .iter()
        .filter(|c| c.num_vertices == 3)
        .count();
    
    stats.largest_five_component_counts = components
        .iter()
        .take(5)
        .map(|c| c.num_vertices)
        .collect();

    stats
}

#[derive(Debug)]
pub struct Component {
    pub number: usize,
    pub num_vertices: usize,
}

/// Analyze the components in the graph, returning a map of component -> num vertices
/// sorted by number of vertices in the components, descending.
fn components(graph: &Graph) -> Vec<Component> {
    let mut map = HashMap::<usize, usize>::new();

    for v in &graph.vertices {
        let entry = map.entry(v.component).or_insert(0);
        *entry += 1;
    }

    let mut v: Vec<_> = map
        .iter()
        .map(|(&number, &num_vertices)| Component { number, num_vertices })
        .collect();

    v.sort_unstable_by(|a, b| b.num_vertices.cmp(&a.num_vertices));
    v
}
