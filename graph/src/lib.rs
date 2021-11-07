use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
use std::{collections::HashMap, fs::File, path::Path};

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
        let f = File::open(filename).unwrap();
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

        Ok(graph)
    }

    fn add_vertex<S: Into<String>>(&mut self, word: S) {
        let word = word.into();

        let vertex = word.clone().into();
        self.vertices.push(vertex);

        self.word_to_index
            .insert(word.into(), self.vertices.len() - 1);
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
        }

        next_component_number += 1;
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
