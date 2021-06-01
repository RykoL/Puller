use std::io;
use std::path::{PathBuf};
use std::process::Command;

fn main() -> io::Result<()> {

    let cwd = std::env::current_dir()?;

    let paths: Vec<PathBuf> = std::fs::read_dir(cwd)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(is_git_repo)
        .collect();

    for repo in paths {
        match pull_repository(&repo) {
            Ok(_) => println!("{}: ✅ Succeeded", repo.display()),
            Err(_) => println!("{}: ❌ Failed", repo.display()),
        }
    }


    Ok(())
}

fn is_git_repo(base_path: &PathBuf) -> bool {
    base_path.join(".git").exists()
}

fn pull_repository(repository: &PathBuf) -> io::Result<()> {
    Command::new("git")
        .args(&["-C", repository.to_str().unwrap(), "pull",])
        .output()
        .and_then(|output| {
            println!("{}", String::from_utf8_lossy(&output.stderr));
            Ok(output)

        })
        .and_then(|output| match output.status.success() {
            true => Ok(()),
            false => Err(io::Error::new(io::ErrorKind::Other, "Failed to pull"))
        })
}
