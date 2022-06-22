use std::env;
use std::fs;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let text = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", text);

    let lines = fs::read_to_string(text).expect("Can't read file.");
    println!("lines after reading and storing from the text:\n{}", lines);
    let chunks: Vec<_> = lines.split(' ').collect();
    let y = chunks.join("");
    let n=y.len();
    println!("strings joined : {}\n of length {}", y,n);
    let m = "m";
    let b="b";
    let rep = lines.replace(&m, &b);
    println!("lines after replacing:\n{}", rep);
}
