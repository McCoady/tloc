use std::path::PathBuf;
use std::fs::{read_dir, read_to_string, File};
use std::io::Write;

pub fn traverse_dir(
    path: &PathBuf,
    file :&mut File,
    traverse: &bool,
    level: usize
) -> u32 {
    let mut code_count = 0;
    // First handle all the files at this level
    for entry in read_dir(path).expect("failed to read path") {
        let entry = entry.expect("failed to handle");
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "sol" {
                    // Process the file
                    code_count += write_output(file, &entry.path());                
                }
            }
        }
    }
    // Then traverse through other directories
    if *traverse {
        for entry in read_dir(path).expect("failed to read path") {
            let entry = entry.expect("failed to handle file");
            let path = entry.path();
            if path.is_dir() {
                let dir_name = &path.file_name().unwrap().to_string_lossy();
                match dir_name.to_lowercase() {
                    name if name == "interfaces" => continue,
                    name if name == "test" => continue,
                    name if name == "tests" => continue,
                    name if name == "mock" => continue,
                    _ => {
                        let header = "#".repeat(level);
                        writeln!(file, "{header} {dir_name}").unwrap();
                        code_count += traverse_dir(&path, file, &true, level+1);
                    },
                }
            }
        }
    }
    code_count
}

fn write_output(file: &mut File, entry: &PathBuf) -> u32 {
    let content = read_to_string(entry).unwrap();
    let mut code_count = 0;

    let mut multi_line_comment = false;
    for line in content.lines() {
        // Handle being inside a multiline comment
        if multi_line_comment {
            if line.contains("*/") {
                multi_line_comment = false;
            }
            continue;
        } 
        match line.trim() {
            val if val.len() == 0 => continue,
            val if val.starts_with("//") => continue,
            val if val.starts_with("/*") && val.ends_with("*/") => continue,
            val if val.starts_with("/*") => {
                multi_line_comment = true;
                continue;
            },
            _ => code_count += 1,
        }
    }

    // Write - `FileName.sol` (COUNT) to file
    let file_name = &entry.file_name().unwrap().to_string_lossy();
    let output = format!("- `{}` ({})", &file_name, code_count);
    writeln!(file, "{}", output).unwrap();

    code_count
}
