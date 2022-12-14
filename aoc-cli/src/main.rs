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
    let mut path = Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).to_path_buf();
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

fn download_input(year: u32, day: u32) -> Result<String, String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let session = std::env::var("AOC_SESSION").map_err(|_| {
        "Session cookie not available. To download input files set AOC_SESSION env variable"
            .to_string()
    })?;

    let resp = reqwest::blocking::Client::new()
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .unwrap();

    assert!(resp.status().is_success());
    Ok(resp.text().unwrap())
}

fn extract_test(page_source: String) -> Option<String> {
    let pat_test = regex::Regex::new(r#"<pre><code>([\s\S]*?)</code></pre>"#).unwrap();

    let mut test_stream = pat_test
        .captures_iter(&page_source)
        .map(|cap| cap.get(1).unwrap().as_str().to_string())
        .map(|data| html_escape::decode_html_entities(&data).to_string());

    test_stream.next()
}

fn download_test(year: u32, day: u32) -> Option<String> {
    // Create folder .aoc_cache if it does not exist
    let cache_dir = project_dir().join(".aoc_cache");
    if !cache_dir.exists() {
        std::fs::create_dir(&cache_dir).unwrap();
    }

    // Try to download the page source from the cache first
    let cache_file = cache_dir.join(format!("{}.html", day));
    let page_source = if cache_file.exists() {
        std::fs::read_to_string(&cache_file).unwrap()
    } else {
        let url = format!("https://adventofcode.com/{}/day/{}", year, day);
        let resp = reqwest::blocking::Client::new().get(&url).send().unwrap();
        assert!(resp.status().is_success());

        // Save the page source to the cache
        let page_source = resp.text().unwrap();
        std::fs::write(&cache_file, &page_source).unwrap();
        page_source
    };

    extract_test(page_source)
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
                .then_some(())
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
                let day = name[3..name.len() - 1].parse().unwrap();
                match download_input(YEAR, day) {
                    Ok(input) => std::fs::write(path.join("input.txt"), input).unwrap(),
                    Err(err) => println!("{}", err),
                }

                match download_test(YEAR, day) {
                    Some(test) => std::fs::write(path.join("test.txt"), test).unwrap(),
                    None => println!("Test not available"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test(day: u32) -> String {
        let file = project_dir().join(format!("day{}a", day)).join("test.txt");
        std::fs::read_to_string(file)
            .unwrap()
            .trim_matches('\n')
            .to_string()
    }

    fn load_all_tests() -> Vec<(u32, String)> {
        (1..=25)
            .filter_map(|day| {
                project_dir()
                    .join(format!("day{}a", day))
                    .exists()
                    .then(|| {
                        dbg!(format!("Loading {}", day));
                        (day, load_test(day))
                    })
            })
            .collect()
    }

    #[test]
    fn test_download_test() {
        for (day, test_in) in load_all_tests() {
            dbg!(format!("Checking test {}", day));
            let parsed_test_in = download_test(2022, day);
            assert_eq!(
                parsed_test_in.unwrap().trim_matches('\n').to_string(),
                test_in
            );
        }
    }
}
