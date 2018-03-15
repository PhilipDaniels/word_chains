use std::collections::{HashMap, HashSet};

pub struct Vertex<'word> {
    word: &'word String,
    adjacency_list: Vec<usize>,
    pub component: Option<usize>
}

impl<'word> Vertex<'word> {
    fn new(word: &'word String) -> Self {
        Vertex {
            word: word,
            adjacency_list: Vec::new(),
            component : None
        }
    }

    fn word(&self) -> &String {
        &self.word
    }
}

pub struct Graph<'word> {
    vertices: Vec<Vertex<'word>>,
    word_to_index: HashMap<&'word String, usize>,
}

impl<'word> Graph<'word> {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            word_to_index : HashMap::new()
        }
    }

    pub fn add_anchor_word(&mut self, word: &'word String) {
        let v = Vertex::new(word);
        self.vertices.push(v);
        self.word_to_index.insert(word, self.vertices.len() - 1);
    }

    pub fn add_reachable_word(&mut self, anchor_word: &'word String, reachable_word: &'word String) {
        let anchor_index = self.word_to_index[anchor_word];
        let reachable_word_index = self.word_to_index[reachable_word];
        let vertex = &mut self.vertices[anchor_index];
        vertex.adjacency_list.push(reachable_word_index);
    }

    pub fn calculate_components(&mut self) {
        let mut next_component_number = 0;

        while let Some(idx) = self.vertices.iter().position(|v| v.component.is_none()) {
            // First find all the vertices in this component and record their indexes in `seen`.
            // This is a read-only operation.
            let mut seen = HashSet::new();
            self.dfs(&mut seen, idx);

            // Then set the component on all those nodes.
            for i in 0..self.vertices.len() {
                if seen.contains(&i) {
                    self.vertices[i].component = Some(next_component_number);
                }
            }

            if seen.len() > 2 {
                println!("Set component number of {} on {} vertices", next_component_number, seen.len());
            }

            next_component_number += 1;
        }
    }

    fn dfs(&self, seen: &mut HashSet<usize>, idx: usize) {
        seen.insert(idx);
        let v = &self.vertices[idx];

        for i2 in &v.adjacency_list {
            if seen.contains(&i2) {
                continue;
            }

            let v2 = &self.vertices[idx];
            if v2.component.is_none() {
                self.dfs(seen, *i2);
            }
        }
    }
}
