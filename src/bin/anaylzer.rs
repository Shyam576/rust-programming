use clap::{Arg, ArgAction, Command};
use std::fs;

fn main() {
    let matches = Command::new("Analyzer")
        .version("1.0")
        .about("Counts lines, words, characters, sentences in a tet file")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Path to the input file")
                .required(true)
                .num_args(1)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("words")
                .long("words")
                .help("Count words")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lines")
                .long("lines")
                .help("Count lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("chars")
                .long("chars")
                .help("Count characters")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sentences")
                .long("sents")
                .help("Count sentences")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let file_path = matches.get_one::<String>("file").unwrap();

    let content = fs::read_to_string(file_path).expect("Failed to read file. Check the file path.");

    if *matches.get_one::<bool>("lines").unwrap_or(&false) {
        println!("Lines: {}", content.lines().count());
    }

    if *matches.get_one::<bool>("words").unwrap_or(&false) {
        println!("Words: {}", content.split_whitespace().count());
    }

    if *matches.get_one::<bool>("chars").unwrap_or(&false) {
        println!("Characters: {}", content.chars().count());
    }

    if *matches.get_one::<bool>("sentences").unwrap_or(&false) {
        let sentence_count = content.matches(|c| c == '.' || c == '!').count();
        println!("Sentences: {}", sentence_count);
    }
}

//cargo run --bin anaylzer -- -f random.txt --words --lines --chars
