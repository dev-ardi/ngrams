#![feature(hash_raw_entry)]
use std::collections::HashMap;

type Idx = u16;

struct TwoGram(pub (Idx, Idx));
impl TwoGram {
}


use std::collections::hash_map::RawEntryMut;

use rustc_hash::FxHashMap;
use unicode_segmentation::UnicodeSegmentation;

fn add_to_corpus(frequencies: &mut FxHashMap<Box<str>, u64>, corpus: &mut str) {
    corpus.make_ascii_lowercase();

    for word in corpus.unicode_words() {
        match frequencies.raw_entry_mut().from_key(word) {
            RawEntryMut::Occupied(o) => {
                *o.into_mut() += 1;
            }
            RawEntryMut::Vacant(v) => {
                v.insert(word.into(), 1);
            }
        }
    }
}
fn main() {

    let corpus = String::from("test");
    let corpus = corpus
        .split(' ')
        .map(strip_non_alphanumeric)
        .filter(String::is_empty);

    let mut words : HashMap<String, usize> = HashMap::new();

    for word in corpus {
        *words.entry(word).or_insert(0) +=1;
    }
}   
