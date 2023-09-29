
use glob::glob;
use std::{fs, io::ErrorKind};
use dirs;
use std::collections::HashSet;
use std::result::Result;
fn main() {
    println!("Hello, world!");
    // list_files() ;
    search_git()
    // get_user_home_directory()
}
 
fn search_git() {
    // Get the user's home directory.
    let home_dir = dirs::home_dir().expect("Failed to get home directory");

    // Construct the search pattern.
    let pattern = format!("{}/**/*/.git", home_dir.display());

    let mut seen_paths = HashSet::new();

    for entry in glob(&pattern).expect("Failed to read glob pattern").filter_map(|entry| entry.ok()) {
        let absolute_path = fs::canonicalize(&entry).unwrap();

        // Skip directories starting with a period or named ".git".
        let file_name = absolute_path.file_name().unwrap().to_string_lossy();
        // if file_name.starts_with('.') || file_name == ".git" {
        //     continue;
        // }

        // Skip the "Library" directory on macOS.
        if cfg!(target_os = "macos") && absolute_path.to_string_lossy().contains("Library") {
            continue;
        }

        if !seen_paths.contains(&absolute_path) {
            println!("{}", absolute_path.display());
            seen_paths.insert(absolute_path);
        }
    }
}

fn handle_glob_error(error: glob::GlobError) {
    let inner_error = error.error();
    if inner_error.kind() != ErrorKind::PermissionDenied {
        println!("Error: {:?}", error);
    }
    else {
        println!("Error: {:?}", error);
    }
}

fn can_read_children(dir: &std::path::Path) -> bool {
    match fs::read_dir(dir) {
        Ok(_) => true,   // If we can read the directory contents, return true.
        Err(_) => false, // If we can't read the directory contents, return false.
    }
}