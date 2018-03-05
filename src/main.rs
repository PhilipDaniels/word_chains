mod dictionary;
mod reachable_words;

fn main() {
    dictionary::create_merged_dictionary();
    reachable_words::calculate_reachable_words();
}
