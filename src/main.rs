use clap::Parser;
use colored::*;
use std::path::Path;

/// Simple program to search for a string either in the name or in the contents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Term to search for
    #[arg(required = true)]
    search_term: String,

    /// Directory to begin the search
    #[arg(short, long, default_value = "{Current Directory}")]
    directory: String,

    /// Should the program search the contents of the files [default:false]
    #[arg(short, long, default_value = "false")]
    contents: bool,
}

fn main() {
    let args = Args::parse();

    let root_directory = std::env::current_dir().unwrap();
    let mut root_directory = root_directory.to_str().unwrap();
    if "{Current Directory}" != args.directory {
        root_directory = Path::new(&args.directory).to_str().unwrap()
    }

    search_directory(&args.search_term, root_directory, args.contents);
    // search
}

fn search_directory(search_term: &str, directory: &str, should_search_contents: bool) {
    let dir_reader = std::fs::read_dir(directory);

    match dir_reader {
        Ok(dir_reader) => {
            for dir_entry in dir_reader {
                match dir_entry {
                    Ok(dir_entry) => {
                        let meta_data = dir_entry.metadata();
                        match meta_data {
                            Ok(meta_data) => {
                                search_filename(search_term, dir_entry.path().to_str().unwrap());
                                if meta_data.is_dir() {
                                    search_directory(
                                        search_term,
                                        dir_entry.path().to_str().unwrap(),
                                        should_search_contents,
                                    )
                                } else if should_search_contents {
                                    search_file_contents(
                                        search_term,
                                        dir_entry.path().to_str().unwrap(),
                                    )
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        _ => {}
    }
}

fn search_file_contents(search_term: &str, file_path: &str) {
    let path = Path::new(file_path);
    let file_contents = std::fs::read_to_string(path);

    match file_contents {
        Ok(file_contents) => {
            let index = file_contents.find(search_term);
            match index {
                None => {}
                Some(_) => {
                    println!("{}", file_path.yellow());
                    parse_lines(&file_contents, search_term);
                }
            }
        }
        Err(_) => {}
    }
}

fn parse_lines(contents: &str, search_term: &str) {
    let lines = contents.lines();
    let count = contents.lines().count();

    let mut is_minified = false;
    for (index, line) in lines.clone().enumerate() {
        if line.len() > 300 {
            is_minified = true;
        } else {
            let search_term_index = line.find(search_term);
            match search_term_index {
                Some(_) => {
                    let start = if index < 3 { 0 } else { index - 2 };
                    let end = if index + 2 > count {
                        count - 1
                    } else {
                        index + 2
                    };

                    println!("line: {}\n{}", index,
"--------------------------------------------------------------------------------".blue()
);

                    let sub_lines: Vec<_> =
                        lines.clone().skip(start).take((end - start) + 1).collect();
                    for line in sub_lines {
                        highlight_found_search(line, search_term);
                    }
                }
                None => {}
            }
        }
    }

    if is_minified {
        println!(
            "{} was found in this file, but the size of the match line prevents displaying",
            search_term,
        );
    }

    println!(
        "{}",
        "--------------------------------------------------------------------------------".blue()
    );
    println!("\n\n")
}

fn highlight_found_search(line: &str, search_term: &str) {
    let index = line.find(search_term);
    match index {
        None => {
            println!("{}", line)
        }
        Some(index) => {
            let start = &line[0..index];
            let middle = search_term.green().bold();
            let end = index + &search_term.len();
            let end = &line[end..];

            println!("{start}{middle}{end}")
        }
    }
}

fn search_filename(search_term: &str, file_path: &str) {
    let path = Path::new(file_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let index = file_name.find(search_term);

    match index {
        Some(index) => {
            let path_without_name = &path.to_str().unwrap();
            let sep = std::path::MAIN_SEPARATOR;
            let path_without_name_index = path_without_name.rfind(|x| x == sep).unwrap();
            let path_without_name = &path_without_name[..path_without_name_index];
            let start = &file_name[0..index];
            let middle = search_term.green().bold();
            let end = index + &search_term.len();
            let end = &file_name[end..];
            println!("{path_without_name}{sep}{start}{middle}{end}");
        }
        None => (),
    };
}
