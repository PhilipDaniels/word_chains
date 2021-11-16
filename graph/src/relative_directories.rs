use std::path::PathBuf;

/// Calculates directories relative to the dictionary directory.
#[derive(Debug)]
pub struct RelativeDirectories {
    dictionary_directory: PathBuf
}

impl RelativeDirectories {
    /// Creates a value based on a dictionary directory.
    pub fn new<P: Into<PathBuf>>(dictionary_directory: P) -> Self {
        Self {
            dictionary_directory: dictionary_directory.into()
        }
    }

    /// Returns the dictionary directory that serves as the base directory.
    pub fn dictionary_directory(&self) -> PathBuf {
        self.dictionary_directory.to_owned()
    }

    /// Returns the directory into which output results
    /// are to be stored.
    pub fn output_directory(&self) -> PathBuf {
        let mut pb = self.dictionary_directory.parent().unwrap().to_path_buf();
        pb.push("output");
        pb
    }

    /// Returns the filename of the corpus file.
    pub fn corpus_file(&self) -> PathBuf {
        let mut pb = self.output_directory();
        pb.push("corpus.txt");
        pb
    }

    /// Returns the name of the 'all adjacenies' file for a particular word length.
    pub fn all_adjacency_file(&self, word_length: usize) -> PathBuf {
        let mut pb = self.output_directory();
        pb.push(format!("all_adjacency_lists_{:02}.txt", word_length));
        pb
    }

    /// Returns the name of the file which will be used to hold the adjacency
    /// lists which construct the largest component.
    pub fn largest_component_adjacency_file(&self, word_length: usize) -> PathBuf {
        let mut pb = self.output_directory();
        pb.push(format!("largest_component_adjacency_lists_{:02}.txt", word_length));
        pb
    }

    /// Returns the name of the file which will hold word length statistics
    /// computed from the graph.
    pub fn word_stats_file(&self) -> PathBuf {
        let mut pb = self.output_directory();
        pb.push("word_stats.csv");
        pb
    }

    /// Returns the name of the 'chains' directory for a specified word length.
    pub fn chains_directory(&self, word_length: usize) -> PathBuf {
        let mut pb = self.output_directory();
        pb.push(format!("chains_{:02}", word_length));
        pb
    }
}
