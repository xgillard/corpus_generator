//! This is the micro service which generates the dynamic corpus for one given
//! student. A corpus is generated by issuing a request to:
//!
//! http:://{server}/linfo/corpus/<secret>/<corpus_id>/<size>/<username>
//!
//! Where:
//! * `{server}` stands for the ip address/dns name of the machine where
//!   this service is deployed.
//! * `<secret>` is a key which is changed each academic year (actually, it does
//!   not even need to be secret). Its purpose is to make sure that a student
//!   taking the same class and project several years in a row receives different
//!   corpora each year. This makes the inginous tasks easier to reuse over time.
//! * `<corpus_id>` is the name of a base corpus (one that has been placed in
//!   the `public/corpus` folder). **WITHOUT ITS .TXT EXTENSION**. For instance,
//!   if I want to generate a subset of the `public/corpus/p1_train.txt` base
//!   corpus, I need to specify the `p1_train` corpus key.
//! * `<size>` is the size (in number of lines) of the corpus you want to generate
//! * `<username>` is the name of the user whose corpus is to be generated.
//!
//! Author: X. Gillard
//! Date: September 29th, 2021

use bzip2::{write::BzEncoder, Compression};
use rocket::fs::NamedFile;
use std::io::Write;

mod corpus;
mod cors;
mod error;
mod named_binary;

use crate::corpus::{gen_corpus_data, pick_random_word};
use crate::cors::Cors;
use crate::error::Result;
use crate::named_binary::{NamedBinary, NamedBinaryBuilder};

use structopt::StructOpt;

/// This is the micro service which generates the dynamic corpus for one given
/// student. A corpus is generated by issuing a request to:
///
/// http:://{server}/bz2/<secret>/<corpus_id>/<username>/<size>/<whatever_filename>
///
/// or (to get the corpus in plain text rather than bz2)
///
/// http:://{server}/txt/<secret>/<corpus_id>/<username>/<size>/<whatever_filename>
///
/// Where:
/// * `{server}` stands for the ip address/dns name of the machine where
///   this service is deployed.
/// * `<secret>` is a key which is changed each academic year (actually, it does
///   not even need to be secret). Its purpose is to make sure that a student
///   taking the same class and project several years in a row receives different
///   corpora each year. This makes the inginous tasks easier to reuse over time.
/// * `<corpus_id>` is the name of a base corpus (one that has been placed in
///   the `public/corpus` folder). **WITHOUT ITS .TXT EXTENSION**. For instance,
///   if I want to generate a subset of the `public/corpus/p1_train.txt` base
///   corpus, I need to specify the `p1_train` corpus key.
/// * `<size>` is the size (in number of lines) of the corpus you want to generate
/// * `<username>` is the name of the user whose corpus is to be generated.
/// * `<whatever_filename>` is the name of the file if it were to be downloaded
#[derive(StructOpt)]
struct Args {
    /// This is the directory where the service will go fetch the data that needs
    /// to be customized before being returned to the students
    #[structopt(long, short, default_value = "public/corpus")]
    corpus_dir: String,
    /// Should we allow cross origin resource sharing (CORS) ? If you intend
    /// to display some content generated by this server through javascript,
    /// then you probably
    #[structopt(long, short = "C")]
    cors: bool,
}

/// This is the global variable which is used to determine the directory
/// where corpora are to be found. (Note: because the directory can be changed
/// through the command line; this variable has to be marked as mutable.
/// However, since it is a global variable, this implies that any access to the
/// variable must occur within an `unsafe` block. Which is expected and totally
/// fine in this case).
static mut CORPUS_DIR: &str = "public/corpus";

/// Returns the filename corresponding to a given (corpus) id
fn fname_from_id(corpus_id: &str) -> String {
    // This block is marked
    unsafe { format!("{}/{}.txt", CORPUS_DIR, corpus_id) }
}

/// This is the implementation of the http endpoint that generates and
/// compresses the customized corpus. This endpoint is mapped to:
///
/// http:://{server}/bz2/<secret>/<corpus_id>/<username>/<size>/<whatever_filename>
///
/// Where:
/// * `{server}` stands for the ip address/dns name of the machine where
///   this service is deployed.
/// * `<secret>` is a key which is changed each academic year (actually, it does
///   not even need to be secret). Its purpose is to make sure that a student
///   taking the same class and project several years in a row receives different
///   corpora each year. This makes the inginous tasks easier to reuse over time.
/// * `<corpus_id>` is the name of a base corpus (one that has been placed in
///   the `public/corpus` folder). **WITHOUT ITS .TXT EXTENSION**. For instance,
///   if I want to generate a subset of the `public/corpus/p1_train.txt` base
///   corpus, I need to specify the `p1_train` corpus key.
/// * `<size>` is the size (in number of lines) of the corpus you want to generate
/// * `<username>` is the name of the user whose corpus is to be generated.
/// * `<whatever_filename>` is the name of the file if it were to be downloaded
///
#[rocket::get("/<secret>/<corpus_id>/<uname>/<size>/<whatever_filename>")]
async fn gen_corpus_bz2(
    secret: &str,
    corpus_id: &str,
    uname: &str,
    size: usize,
    whatever_filename: &str,
) -> Result<NamedBinary> {
    let fname = fname_from_id(corpus_id);
    let mut buffer = vec![];
    let data = gen_corpus_data(secret, &fname, uname, size).await?;
    let mut encoder = BzEncoder::new(&mut buffer, Compression::best());
    encoder.write_all(data.as_bytes())?;
    let gzipped = encoder.finish()?;
    let responder = NamedBinaryBuilder::default()
        .content_type("application/bzip2")
        .download_name(whatever_filename.to_string())
        .payload(gzipped.to_vec())
        .build()?;
    Ok(responder)
}

/// This is the implementation of the http endpoint that generates a plain text
/// customized corpus. This endpoint is mapped to:
///
/// http:://{server}/txt/<secret>/<corpus_id>/<username>/<size>/<whatever_filename>
///
/// Where:
/// * `{server}` stands for the ip address/dns name of the machine where
///   this service is deployed.
/// * `<secret>` is a key which is changed each academic year (actually, it does
///   not even need to be secret). Its purpose is to make sure that a student
///   taking the same class and project several years in a row receives different
///   corpora each year. This makes the inginous tasks easier to reuse over time.
/// * `<corpus_id>` is the name of a base corpus (one that has been placed in
///   the `public/corpus` folder). **WITHOUT ITS .TXT EXTENSION**. For instance,
///   if I want to generate a subset of the `public/corpus/p1_train.txt` base
///   corpus, I need to specify the `p1_train` corpus key.
/// * `<size>` is the size (in number of lines) of the corpus you want to generate
/// * `<username>` is the name of the user whose corpus is to be generated.
/// * `<whatever_filename>` is the name of the file if it were to be downloaded
///
#[rocket::get("/<secret>/<corpus_id>/<uname>/<size>/<_whatever_filename>")]
async fn gen_corpus_txt(
    secret: &str,
    corpus_id: &str,
    uname: &str,
    size: usize,
    _whatever_filename: &str,
) -> Result<String> {
    let fname = fname_from_id(corpus_id);
    let data = gen_corpus_data(secret, &fname, uname, size).await?;
    Ok(data)
}

/// This endpoint lets you pick a random word from a static corpus.
/// This endpoint is mapped to:
///
/// http:://{server}/word/<secret>/<corpus_id>/<username>/<min_occurs>/<min_length>/<nth_random_word>
///
/// Where:
/// * `{server}` stands for the ip address/dns name of the machine where
///   this service is deployed.
/// * `<secret>` is a key which is changed each academic year (actually, it does
///   not even need to be secret). Its purpose is to make sure that a student
///   taking the same class and project several years in a row receives different
///   corpora each year. This makes the inginous tasks easier to reuse over time.
/// * `<corpus_id>` is the name of a base corpus (one that has been placed in
///   the `public/corpus` folder). **WITHOUT ITS .TXT EXTENSION**. For instance,
///   if I want to generate a subset of the `public/corpus/p1_train.txt` base
///   corpus, I need to specify the `p1_train` corpus key.
/// * `<blacklist>` is the name of a text file (located in the same directory as
///   the corpora). That file contains one line per word and each of these words
///   constitute the black list. Those words are never going to be considered
///   when producing a random sample for the student. 
/// * `<username>` is the name of the user whose corpus is to be generated.
/// * `<min_occurs>` is the minimum number of times the word must appear in the
///    corpus in order to be considered a possible candidate.
/// * `<min_length>` is the minimum length of a word that can be considered.
/// * `<nth_random_word>` is used to distinguish several calls to the service
///    and generate different words in each case.
///
#[rocket::get("/<secret>/<corpus_id>/<blacklist>/<uname>/<min_occurs>/<min_length>/<nth_random_word>")]
async fn random_word_from_static_corpus(
    secret: &str,
    corpus_id: &str,
    blacklist: &str, 
    uname: &str,
    min_occurs: usize,
    min_length: usize,
    nth_random_word: usize,
) -> Result<String> {
    let corpus = fname_from_id(corpus_id);
    let forbid = fname_from_id(blacklist);
    let data = pick_random_word(secret, &corpus, &forbid, uname, min_occurs, min_length, nth_random_word).await?;
    Ok(data)
}

/// This endpoint serves a static file
#[rocket::get("/<corpus_id>")]
async fn static_file(corpus_id: &str) -> Result<NamedFile> {
    let fname = fname_from_id(corpus_id);
    Ok(NamedFile::open(fname).await?)
}

/// This is the program's main entry point. It spawns the server whenever it
/// gets run
#[rocket::main]
async fn main() {
    #[allow(unused_assignments)]
    let mut cors = false;
    // This is the short portion of code where I might write to the CORPUS_DIR
    // global variable. It happens **before** any request has a chance of
    // getting processed. This is why this block is guaranteed to be safe
    unsafe {
        let args = Args::from_args();
        let cdir = Box::new(args.corpus_dir);
        CORPUS_DIR = Box::leak(cdir);
        cors = args.cors;
    }

    let mut spaceship = rocket::build()
        .mount("/static", rocket::routes![static_file])
        .mount("/bz2", rocket::routes![gen_corpus_bz2])
        .mount("/txt", rocket::routes![gen_corpus_txt])
        .mount("/word", rocket::routes![random_word_from_static_corpus]);

    if cors {
        spaceship = spaceship.attach(Cors);

    }

    spaceship.launch().await.expect("unexpected failure");
}
