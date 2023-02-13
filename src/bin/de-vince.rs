// Let the crate be named what we want!
#![allow(non_snake_case)]

use VanGoghDec::{VanGoghDecoder};

fn random_quote() -> String {
    let q = ["\"It is good to love many things, for therein lies the true strength, and whosoever loves much performs much, and can accomplish much, and what is done in love is well done.\" --VVG",
    "\"I dream my painting and I paint my dream.\" --VVG",
    "\"Be clearly aware of the stars and infinity on high. Then life seems almost enchanted after all.\" --VVG",
    "\"There is nothing more truly artistic than to love people.\" --VVG",
    "\"A great fire burns within me, but no one stops to warm themselves at it, and passers-by only see a wisp of smoke\" --VVG",
    "\"I don't know anything with certainty, but seeing the stars makes me dream.\" --VVG",
    "\"Normality is a paved road: It’s comfortable to walk, but no flowers grow on it.\" --VVG",
    "\"If you hear a voice within you say you cannot paint, then by all means paint and that voice will be silenced.\" --VVG",
    "\"I put my heart and soul into my work, and I have lost my mind in the process.\" --VVG",
    "\"I often think that the night is more alive and more richly colored than the day.\" --VVG"];

    let a = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() % 10;

    return String::from(q[a as usize]);
}

fn main() {
    // format is `Van
    let args: Vec<_> = std::env::args().collect();
    let img = VanGoghDecoder::decode_path(&args[1]);
    
    img.to_png(&args[2]);

    println!("{}", random_quote());
}
