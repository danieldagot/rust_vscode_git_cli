use clap::{Command, Arg, ValueHint, value_parser, ArgAction};
use clap_complete::{generate, Generator, Shell};
use std::io;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn build_cli() -> Command {
    Command::new("search-git")
        .arg(Arg::new("directory")
            .short('d')  // <-- Add this line
            .long("directory") // <-- Add this line
            .required(true) 
            .value_hint(ValueHint::AnyPath))
        .arg(Arg::new("generator")
            .long("generate")
            .action(ArgAction::Set)
            .value_parser(value_parser!(Shell)))
}


fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

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
            if let Some(parent) = entry.path().parent() {
                git_repositories.push(parent.to_path_buf());
            }
        }
    }

    git_repositories
}

fn main() {
    let matches = build_cli().get_matches();

    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = build_cli();
        eprintln!("Generating completion file for {generator}...");
        print_completions(generator, &mut cmd);
    } else if let Some(directory) = matches.get_one::<String>("directory") {

        let start_dir = Path::new(directory);
        let git_repositories = search_git_repositories(&start_dir);
        for repo_path in &git_repositories {
            println!("{}", repo_path.display());
        }
    }
}
