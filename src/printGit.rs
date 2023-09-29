use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn is_not_hidden(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_str().unwrap_or("");
    if file_name == ".git" {
        return true;
    }
    entry.depth() == 0 || !file_name.starts_with(".")
}

fn is_git_repository(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s == ".git")
        .unwrap_or(false)
}

fn search_git_repositories(root_path: &Path) -> Vec<PathBuf> {
    let mut git_repositories = Vec::new();

    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
    {
        if is_git_repository(&entry) {
            // Get the parent directory and push it to the vector
            if let Some(parent) = entry.path().parent() {
                git_repositories.push(parent.to_path_buf());
            }
        }
    }

    git_repositories
}

fn main() {
    let start_dir = Path::new("/Users/daniedagot");

    if let Err(err) = std::env::set_current_dir(&start_dir) {
        eprintln!("Error setting current directory: {:?}", err);
        return;
    }

    let git_repositories = search_git_repositories(&start_dir);

    // Print the parent directories of Git repositories
    for repo_path in &git_repositories {
        println!("{}", repo_path.display());
    }
}
