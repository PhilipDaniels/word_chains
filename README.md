# Running Order

## dictionary_merger
This will amalgamate all the word files in 'dictionaries' and create 'corpus.txt' in dictionaries_out', which is a file containing all the cleaned words from each of the dictionaries. 

## calc_adjacency_lists
This will create a set of files 'adjacency_list_NN.txt' which contain all the adjacency lists for each word in the corpus. These files are the input into the graph calculation.

## word_chain_calculator
This will calculate the word graphs for each word length and
write a summary file 'word_graph_stats.csv'. It will also create
a new file 'largest_component_NN.txt' which is an adjacency
list for only the largest component in the computed graphs (the
longest word chain is always in the largest component, because
the largest component is always much, much larger than the
second largest component, so we can simplify debugging by
just focusing on the largest component).
