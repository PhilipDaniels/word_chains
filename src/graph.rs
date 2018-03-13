use std::collections::HashMap;

pub struct Vertex {
    word: String,
    adjacency_list: Vec<usize>,
    pub component: usize
}

impl Vertex {
    fn new(word: String) -> Self {
        Vertex { word: word, adjacency_list: Vec::new(), component : 0 }
    }

    fn word(&self) -> &String {
        &self.word
    }
}

pub struct Graph {
    vertices: Vec<Vertex>,
    word_to_index: HashMap<String, usize>,
    components: Vec<usize>
}

impl Graph {
    fn new() -> Self {
        Graph { vertices: Vec::new(), word_to_index : HashMap::new(), components: Vec::new () }
    }

    fn add_anchor_word(word: String) {

    }
}