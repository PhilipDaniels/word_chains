use std::fs;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::io::prelude::*;

use ::dictionary;

pub fn calculate_reachable_words() {
    let f = fs::File::open(dictionary::CORPUS).unwrap();
}