# Running Order

## pre_calc

This program is used to pre-calculate all the adjacency lists
and graph stats. It has command line options, ordered
1, 2 and 3.

#### 1 - dictionary merger
This will amalgamate all the word files in 'dictionaries' and create 'corpus.txt' in 'output'', which is a file containing all the cleaned words from each of the dictionaries. 

#### 2 - adjacency_calculator
This will create a set of files 'all_adjacency_lists_NN.txt' which contain all the adjacency lists for each word in the corpus. These files are the input into the graph calculation.

#### 3 - graph_calculator
This will calculate word graphs for each word length and
write a summary file 'word_stats.csv'. It will also create
a new file 'largest_component_adjacency_lists_NN.txt' which is
an adjacency list for only the largest component in the computed
graphs (the longest word chain is always in the largest component,
because the largest component is always much, much larger than the
second largest component, so we can simplify debugging by just
focusing on the largest component).
