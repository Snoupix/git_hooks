use git2::{Repository, StatusOptions, StatusShow};
use std::path::Path;
use std::process::Command;

fn main() {
    let project_root = Path::new(".");
    let package_json = project_root.join("package.json");

    if !package_json.exists() {
        eprintln!("No package.json found, cannot run linter command");
        std::process::exit(1);
    }

    let output = Command::new("npm")
        .args(["run", "fix"])
        .current_dir(project_root)
        .output()
        .expect("Failed to run NPM fix command");

    if !output.status.success() {
        println!("NPM fix command failed: {:?}", output);
        std::process::exit(1);
    }

    println!("[npm run fix] Files linted");

    let repo = Repository::discover(project_root).expect("Cannot find git repo");
    let mut status_options = StatusOptions::new();
    status_options.show(StatusShow::Workdir);
    let statuses = repo.statuses(Some(&mut status_options)).unwrap();

    if !statuses.is_empty() {
        println!(
            "There are {} modified files not added to commit, commit aborted",
            statuses.iter().count()
        );
        std::process::exit(1);
    }
}
