use std::collections::HashMap;

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
    components: Vec<usize>
}

impl<'word> Graph<'word> {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            word_to_index : HashMap::new(),
            components: Vec::new ()
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

        // New approach. Find first, then mutate.
        // Get a list of all the indexes reachable from a starting index.
        // Then return, and set them all to the same component.

        while let Some(idx) = self.vertices.iter().position(|v| v.component.is_none()) {
            self.depth_first_search(idx, next_component_number);
            next_component_number += 1;
        }

//        for i in 0..self.vertices.len() {
//            let mut v = &self.vertices[i];
//            if v.component.is_some() {
//                continue;
//            }
//            v.component = Some(next_component_number);
//            self.depth_first_search(&mut v);
//            next_component_number += 1;
//        }
    }

    fn depth_first_search(&mut self, idx: usize, component_number: usize) {
        {
            let v = &mut self.vertices[idx];
            v.component = Some(component_number);
        }

        let v = &self.vertices[idx];
        for &i in &v.adjacency_list {
            if self.vertices[i].component.is_none() {
                self.depth_first_search(i, component_number);
            }
        }

        /*
            v.SetComponent(c);
            c.vertex_count++;

            foreach (idx; v.AdjList())
            {
                auto v2 = _vertices[idx];
                if (!v2.GetComponent())
                    DFS(v2, c);
            }
        */
    }
}