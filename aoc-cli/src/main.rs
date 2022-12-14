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

fn download_input(year: u32, day: u32) -> String {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let session = std::env::var("AOC_SESSION").unwrap();

    let resp = reqwest::blocking::Client::new()
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .unwrap();

    assert!(resp.status().is_success());
    resp.text().unwrap()
}

fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();

    const YEAR: u32 = 2022;

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
            let src = if name.ends_with('b') {
                let a_name = name.replace('b', "a");
                let a_path = project_dir().join(&a_name);
                assert!(a_path.exists(), "Problem a does not exist");
                a_name
            } else {
                String::from("template")
            };

            // Create the directory cloning all entries from the template
            copy_dir_all(project_dir().join(&src), &path).unwrap();

            // Update name field in the Cargo.toml to the name of the problem
            let project_toml = path.join("Cargo.toml");
            let mut contents = std::fs::read_to_string(&project_toml).unwrap();
            contents = contents.replace(&src, &name);
            std::fs::write(&project_toml, contents).unwrap();

            // Add the problem name to the Cargo workspace members
            let workspace_toml = project_dir().join("Cargo.toml");
            let mut contents = std::fs::read_to_string(&workspace_toml).unwrap();
            contents = contents.replace("members = [", &format!("members = [\n    \"{}\",", name));
            std::fs::write(&workspace_toml, contents).unwrap();

            // Download the input for the problem if it is a problem a
            if name.ends_with('a') {
                let input = download_input(YEAR, name[3..name.len() - 1].parse().unwrap());
                std::fs::write(path.join("input.txt"), input).unwrap();
            }
        }
    }
}
