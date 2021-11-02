extern crate structopt;

mod dictionary;
mod graph;
mod letter_differences;
mod word_base;
mod word_table;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "word_chains", about = "A program for calculating word chains.")]
struct Opt {
    #[structopt(short = "d", long = "merge-dictionaries", help = "Merges the files in the dictionaries folder into corpus.txt")]
    merge_dictionaries: bool,

    #[structopt(short = "o", long = "one-letter", help = "Calculates all the words that are one letter different from each other")]
    calc_one_letter_differences: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.merge_dictionaries {
        dictionary::create_merged_dictionary();
    }

    if opt.calc_one_letter_differences {
        letter_differences::calculate_words_one_letter_different();
    }
}
