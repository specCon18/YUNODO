use clap::Parser;
use std::{io,fs,path::PathBuf};
//TODO: implement mod file_tree; :ODOT//

#[derive(Parser)]
#[command(name = "YUNODO")]
#[command(version = "0.1.0")]
#[command(about = "parse file tree for //TODO: comments", long_about = "parses a directory of files for substrings of //TODO: and outputs all instances in a parsable format")]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,
    #[arg(short, long, value_name = "DEBUG")]
    debug: Option<bool>,
}

fn read_files_in_directory(dir_path: &str) -> io::Result<Vec<(String, Vec<String>)>> {
    let mut files_content = Vec::new();
    let paths = fs::read_dir(dir_path)?;

    for path in paths {
        let entry = path?;
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_string_lossy().into_owned();
            let content = fs::read_to_string(&path)?;
            let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            files_content.push((filename, lines));
        } else if path.is_dir() {
            let subdir_path = path.to_string_lossy().into_owned();
            let subdir_content = read_files_in_directory(&subdir_path)?;
            files_content.extend(subdir_content);
        }
    }

    Ok(files_content)
}

fn main() {
    let cli = Cli::parse();
    if let Some(path) = cli.path.as_deref() {
        let path_string = path.display().to_string();
        match read_files_in_directory(&path_string.as_str()) {
            Ok(files_content) => {
                for (filename, lines) in files_content {
                    for (line_number, line) in lines.iter().enumerate() {
                        // Check if the line starts with "//" and not within strings or other signatures
                        if !line.contains("//") {
                            continue;
                        }

                        let mut in_string = false;
                        let mut in_comment = false;
                        let mut in_signature = false;

                        for (i, c) in line.chars().enumerate() {
                            match c {
                                '"' => in_string = !in_string,
                                '/' if !in_string => {
                                    if i + 1 < line.len() && line.chars().nth(i + 1).unwrap() == '/' {
                                        in_comment = true;
                                        break;
                                    } else if i + 1 < line.len() && line.chars().nth(i + 1).unwrap() == '*' {
                                        in_comment = true;
                                    }
                                }
                                '*' if i + 1 < line.len() && line.chars().nth(i + 1).unwrap() == '/' && in_comment => {
                                    in_comment = false;
                                    break;
                                }
                                _ => (),
                            }
                        }

                        if in_comment {
                            let mut v: Vec<&str> = line.split("//TODO:").collect();
                            if let Some(last_part) = v.last() {
                                if let Some(end_index) = last_part.find(":ODOT//") {
                                    let extracted = &last_part[..end_index];
                                    println!("({})->({})->({}) |~| {}", path_string, filename, line_number + 1, extracted);
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}
