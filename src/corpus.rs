//! This module contains all the code which truly relates to the actual
//! customized corpus generation.
//!
//! Author: X. Gillard
//! Date: September 29th, 2021

use std::path::Path;

use crate::error::Result;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use rustc_hash::FxHashMap;
use sha2::{Digest, Sha256};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

/// A strong and fast seedable rng
type PRng = Xoshiro256Plus;
type Seed = [u8; 32];

/// Actually generates the corpus data (this is where the most important bits
/// of logic is located... even though it's pretty trivial).
pub async fn gen_corpus_data(
    secret: &str,
    fname: &str,
    uname: &str,
    size: usize,
) -> Result<String> {
    let sample = sample(secret, fname, uname, size);

    let mut corpus = String::new();
    let file = File::open(&fname);
    let read = BufReader::new(file.await?);
    let mut lines = read.lines();
    let sample = sample.await?;
    let mut iter = sample.iter().copied();
    let mut take = iter.next();
    let mut i = 0;

    while let Some(x) = take {
        let line = lines.next_line().await?;
        if let Some(line) = line {
            if i == x {
                corpus.push_str(&line);
                take = iter.next();
                corpus.push('\n');
            }
        } else {
            break;
        }
        i += 1;
    }

    Ok(corpus)
}

pub async fn pick_random_word(
    secret: &str,
    fname: &str,
    uname: &str,
    min_occurs: usize,
    min_length: usize,
    nth_random_word: usize,
) -> Result<String> {
    let mut rng = PRng::from_seed(seed(uname, secret));

    // Open the targetted corpus
    let file = File::open(fname);
    let read = BufReader::new(file.await?);
    let mut lines = read.lines();

    // collect all words in a hash set
    let mut words = FxHashMap::default();
    while let Some(line) = lines.next_line().await? {
        // When picking up random words, from a corpus, I want to make sure
        // to isolate only alphabetic words. This excludes named entities
        // such as X15, but I think it is a fair move to do.
        for word in line.split(|c: char| !c.is_alphabetic()) {
            if word.len() >= min_length {
                words.entry(word.to_lowercase())
                    .and_modify(|e| *e+=1)
                    .or_insert(1);
            }
        }
    }

    // keep only those words occuring more than the required threshold
    words.retain(|_k, v| *v >= min_occurs);

    // pick some random word in the set
    let mut target = rng.gen_range(0..words.len());
    for _ in 0..nth_random_word {
        target = rng.gen_range(0..words.len());
    }

    let chosen = words.into_iter()
        .map(|e| e.0)
        .nth(target)
        .unwrap();
    
    Ok(chosen)
}

/// Creates an unique seed from a given key and user name
/// The returned seed is an array of 32 bytes
fn seed(uname: &str, key: &str) -> Seed {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hasher.update(uname.as_bytes());
    let digest = hasher.finalize();

    let mut seed = [0_u8; 32];
    seed.clone_from_slice(&digest[..32]);
    seed
}

/// Counts the number of lines in the given file
async fn nb_lines<P: AsRef<Path>>(fname: P) -> Result<usize> {
    let file = File::open(fname).await?;
    let mut lines = BufReader::new(file).lines();
    let mut count = 0;
    while lines.next_line().await?.is_some() {
        count += 1;
    }
    Ok(count)
}

/// Returns a sample of the lines that must be selected inside of the generated
/// corpus
async fn sample(secret: &str, fname: &str, uname: &str, size: usize) -> Result<Vec<usize>> {
    let lines = nb_lines(fname);
    let mut rng = PRng::from_seed(seed(uname, secret));
    let mut sample = rand::seq::index::sample(&mut rng, lines.await?, size).into_vec();
    sample.sort_unstable();
    Ok(sample)
}
