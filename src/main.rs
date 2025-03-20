use rand::distr::{Distribution, slice::Choose};
use rand::seq::{IndexedRandom, IteratorRandom};

fn generate_chunk(length: usize) -> String {
    let mut rng = rand::rng();
    let lowers = 'a'..='z';
    let uppers = 'A'..='Z';

    // this feels gross
    [lowers, uppers].choose(&mut rng).map(|pool| {
        let selected_chars = pool.clone().choose_multiple(&mut rng, length);
        selected_chars.iter().collect::<String>()
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

fn generate_password(num_chunks: usize, chunk_size: usize) -> Option<String> {
    let chunks: Vec<String> = (0..num_chunks).map(|_| generate_chunk(chunk_size)).collect();
    let separators: Vec<String> = (0..num_chunks-1).map(|_| random_separator()).collect();

    println!("{:?}", chunks);
    println!("{:?}", separators);
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
    let pw = generate_password(4, 5);
    println!("{:?}", pw);
}
