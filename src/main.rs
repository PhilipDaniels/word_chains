mod dictionary;
mod reachable_words;
mod word_base;
mod word_table;

fn main() {
    //dictionary::create_merged_dictionary();
    reachable_words::calculate_reachable_words();
}
