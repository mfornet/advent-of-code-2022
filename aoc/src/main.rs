use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Clone)]
enum Action {
    Init { name: String },
}

fn project_dir() -> PathBuf {
    let mut path = Path::new(file!()).to_path_buf();
    while path.file_name().unwrap() != "aoc" {
        path.pop();
    }
    path.pop();
    path
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Init { name } => {
            // Check name follows the correct format
            regex::Regex::new(r"^day\d{1,2}[ab]$")
                .unwrap()
                .is_match(&name)
                .then(|| ())
                .unwrap_or_else(|| panic!("Invalid problem name"));

            // Check problem was not already created
            let path = project_dir().join(&name);
            assert!(!path.exists(), "Problem already created");

            // If this is problem b copy everything from problem a
            if name.ends_with('b') {
                let a_name = name.replace('b', "a");
                let a_path = project_dir().join(&a_name);
                assert!(a_path.exists(), "Problem a does not exist");
                copy_dir_all(a_path, &path).unwrap();
            } else {
                // Create the directory cloning all entries from the template
                copy_dir_all(project_dir().join("template"), &path).unwrap();
            }

            // Update name field in the Cargo.toml to the name of the problem
            let project_toml = path.join("Cargo.toml");
            let mut contents = std::fs::read_to_string(&project_toml).unwrap();
            contents = contents.replace("template", &name);
            std::fs::write(&project_toml, contents).unwrap();

            // Add the problem name to the Cargo workspace members
            let workspace_toml = project_dir().join("Cargo.toml");
            let mut contents = std::fs::read_to_string(&workspace_toml).unwrap();
            contents = contents.replace("members = [", &format!("members = [\n    \"{}\",", name));
            std::fs::write(&workspace_toml, contents).unwrap();
        }
    }
}
