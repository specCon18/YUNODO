use clap::Parser;
use std::{io, fs, path::PathBuf, collections::HashMap};

#[derive(Parser)]
#[command(name = "YUNODO")]
#[command(version = "0.5.0")]
#[command(about = "parse file tree for //TODO: comments", long_about = "parses a directory of files for substrings of //TODO: and outputs all instances in a parsable format")]
struct Cli {
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,
    #[arg(short, long, value_name = "FORMAT")]
    format: Option<String>,
    #[arg(short, long)]
    debug: Option<bool>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(path) = cli.path.as_deref() {
        let path_string = path.display().to_string();
        let mut todos: Vec<(String, String, String, String, u8)> = Vec::new();

        match read_files_in_directory(&path_string.as_str()) {
            Ok(files_content) => {
                for (filename, lines) in files_content {
                    for (line_number, line) in lines.iter().enumerate() {
                        if !line.contains("//TODO:") {
                            continue;
                        }

                        let mut in_string = false;
                        let mut in_comment = false;

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
                            if let Some(todo_comment) = extract_todo_comment(line) {
                                let (adjusted_todo_comment, uscore) = extract_uscore(&todo_comment);
                                let line_number_str = (line_number + 1).to_string(); // Convert line number to String
                                
                                todos.push((path_string.clone(), filename.clone(), line_number_str, adjusted_todo_comment, uscore));
                            }
                        }
                    }
                }

                // Sort todos by uscore (descending order)
                todos.sort_by(|(_, _, _, _, uscore1), (_, _, _, _, uscore2)| uscore2.cmp(uscore1));
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }

        if let Some(format) = cli.format.as_deref() {
            match format {
                "md" | "MD" => out_as_md_table(todos),
                "json" | "JSON" => out_as_json_object(todos),
                "yaml" | "YAML" => out_as_yaml_file(todos),
                "toml" | "TOML" => out_as_toml_file(todos),
                _ => println!("That's not a supported format")
            }
        }
    }
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

fn extract_todo_comment(line: &str) -> Option<String> {
    if let Some(start) = line.find("//TODO:") {
        if let Some(end) = line[start + "//TODO:".len()..].find(":ODOT//") {
            return Some(line[start + "//TODO:".len()..start + "//TODO:".len() + end].trim().to_string());
        }
    }
    None
}

fn extract_uscore(comment: &str) -> (String, u8) {
    // Initialize uscore to 0 by default
    let mut uscore = 0;
    
    // Check if comment starts with "U:"
    if let Some(start_uscore) = comment.find("U:") {
        // Calculate the start and end positions for slicing
        let start_idx = start_uscore + "U:".len();
        if let Some(end_uscore) = comment[start_idx..].find(' ') {
            // Extract the number part after "U:"
            let num_str = &comment[start_idx..start_idx + end_uscore];
            // Attempt to parse the substring as u8
            if let Ok(num) = num_str.parse::<u8>() {
                uscore = num;
            }
        }
        // Remove "U: number" part from the comment
        let new_comment = format!("{}{}", &comment[..start_uscore], &comment[start_uscore + "U: ".len()..]);
        return (new_comment, uscore);
    }

    // Return original comment and 0 uscore if "U:" or a valid number is not found
    (comment.to_string(), uscore)
}

fn out_as_md_table(todos: Vec<(String, String, String, String, u8)>) {
    let headers = String::from("| File Path | File Name | Line Number | Comment | Uscore |");
    let divider = String::from("|:----------|:---------:|:-----------:|:--------|:------|");

    let mut table: Vec<String> = Vec::new();
    table.push(headers);
    table.push(divider);

    for (path, filename, line_number, comment, uscore) in todos {
        let formatted_row = format!("| {} | {} | {} | {} | {} |", path, filename, line_number, comment, uscore);
        table.push(formatted_row);
    }

    for line in table {
        println!("{}", line);
    }
}

fn out_as_json_object(todos: Vec<(String, String, String, String, u8)>) {
    let mut output: Vec<String> = Vec::new();
    let object_open_char = "{".to_string();
    let object_close_char = "}".to_string();

    output.push(object_open_char.clone());

    for (path, filename, line_number, comment, uscore) in todos {
        let mut obj_content = String::new();
        obj_content.push_str("    {");
        obj_content.push_str(&format!("\"file_path\": \"{}\", ", path));
        obj_content.push_str(&format!("\"file_name\": \"{}\", ", filename));
        obj_content.push_str(&format!("\"line_number\": \"{}\", ", line_number));
        obj_content.push_str(&format!("\"todo_comment\": \"{}\", ", comment));
        obj_content.push_str(&format!("\"uscore\": \"{}\"", uscore));
        obj_content.push_str("},");
        output.push(obj_content);
    }

    output.push(object_close_char);
    for line in output {
        println!("{}", line)
    }
}

fn out_as_toml_file(todos: Vec<(String, String, String, String, u8)>) {
    let mut todos_map: HashMap<String, Vec<(String, String, u8)>> = HashMap::new();

    for (path, filename, line_number, comment, uscore) in todos {
        let header = format!("{}{}", path, filename);
        todos_map.entry(header.clone()).or_insert(Vec::new()).push((line_number.clone(), comment.clone(), uscore));
    }

    let mut toml_output = String::new();
    for (header, todo_list) in todos_map {
        toml_output.push_str(&format!("[{}]\n", header));

        for (i, (line, comment, uscore)) in todo_list.iter().enumerate() {
            toml_output.push_str("[[todo]]\n");
            toml_output.push_str(&format!("line = {}\n", line));
            toml_output.push_str(&format!("comment = \"{}\"\n", comment));
            toml_output.push_str(&format!("uscore = {}\n", uscore));

            if i < todo_list.len() - 1 {
                toml_output.push_str("\n");
            }
        }
    }
    println!("{}", toml_output);
}

fn out_as_yaml_file(todos: Vec<(String, String, String, String, u8)>) {
    let mut todos_map: HashMap<String, Vec<(String, String, u8)>> = HashMap::new();

    for (path, filename, line_number, comment, uscore) in todos {
        let header = format!("{}{}", path, filename);
        todos_map.entry(header.clone()).or_insert(Vec::new()).push((line_number.clone(), comment.clone(), uscore));
    }

    let mut yaml_output = String::new();
    for (header, todo_list) in todos_map {
        yaml_output.push_str(&format!("\"{}\":\n", header));

        for (line, comment, uscore) in todo_list {
            yaml_output.push_str("    \"item\":\n");
            yaml_output.push_str(&format!("        \"line_number\": \"{}\"\n", line));
            yaml_output.push_str(&format!("        \"comment\": \"{}\"\n", comment));
            yaml_output.push_str(&format!("        \"uscore\": \"{}\"\n", uscore));
        }
    }

    println!("{}", yaml_output);
}
