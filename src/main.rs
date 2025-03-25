use std::num::ParseIntError;
use clap::{Arg, ArgAction, Command};
use clap::builder::ValueParser;
use rand::distr::{Distribution, slice::Choose, Uniform};
use rand::Rng;
use rand::seq::{IndexedRandom};

fn generate_chunk(length: usize) -> String {
    let mut rng = rand::rng();
    let lowers: Vec<char> = ('a'..='z').collect();
    let uppers: Vec<char> = ('A'..='Z').collect();

    // this feels gross
    [lowers, uppers].choose(&mut rng).map(|pool| {
        let pool_indices = Uniform::new(0, pool.len()).unwrap();
        let selected_indices: Vec<usize> = (&mut rng).sample_iter(pool_indices).take(length).collect();
        selected_indices.iter().map(|i| pool[*i]).collect::<String>()
    }).unwrap()
}

fn random_separator() -> String {
    let mut rng = rand::rng();
    let numbers = '0'..='9';
    let special_seps = [
        '!',
        '@',
        '#',
        '$',
        '%',
        '^',
        '&',
        '*',
        '/',
        ':',
        ';',
        '-',
        '_',
        '+',
        '=',
        '~'
    ];
    let sep_pool = numbers.chain(special_seps).collect::<Vec<_>>();
    let sep_rng = Choose::new(sep_pool.as_slice()).unwrap();
    let c = sep_rng.sample(&mut rng);
    String::from(*c)
}

fn parse_usize(env: &str) -> Result<usize, ParseIntError> {
    env.parse::<usize>()
}

fn generate_password(num_chunks: usize, chunk_size: usize) -> Option<String> {
    let chunks: Vec<String> = (0..num_chunks).map(|_| generate_chunk(chunk_size)).collect();
    let separators: Vec<String> = (0..num_chunks-1).map(|_| random_separator()).collect();

    let total_size = chunks.len() + separators.len();

    let mut pw: Vec<String> = Vec::with_capacity(total_size);
    for i in 0..total_size {
        if i % 2 == 0 {
            pw.push(chunks[i / 2].clone());
        } else {
            pw.push(separators[(i-1) / 2].clone());
        }
    }
    Some(pw.join(""))
}

fn main() {
    let matches = Command::new("Disco")
        .author("JDSeiler")
        .version("0.1.0")
        .about("Generates passwords in an easy to read chunked format, inspired by 1Password")
        .arg(
            Arg::new("num_chunks")
                .long("chunks")
                .short('c')
                .value_parser(ValueParser::new(parse_usize))
                .default_value("5")
                .action(ArgAction::Set)
                .help("Number of chunks to generate")
        )
        .arg(
            Arg::new("chunk_size")
                .long("chunk-size")
                .short('s')
                .value_parser(ValueParser::new(parse_usize))
                .default_value("3")
                .action(ArgAction::Set)
                .help("Number of characters per chunk")
        ).get_matches();

    let num_chunks = *matches.get_one::<usize>("num_chunks").unwrap();
    let chunk_size = *matches.get_one::<usize>("chunk_size").unwrap();

    if let Some(pw) = generate_password(num_chunks, chunk_size) {
        println!("{}", pw);
    } else {
        println!("Failed to generate password! This is a bug!")
    }
}
