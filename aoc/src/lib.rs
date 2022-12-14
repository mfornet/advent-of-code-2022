use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub fn input(cur_file: &str) -> BufReader<File> {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input".to_string());
    let mut path = Path::new(cur_file)
        .parent()
        .and_then(|path| path.parent())
        .unwrap()
        .join(input_path);
    path.set_extension("txt");
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
}

pub fn input_str(cur_file: &str) -> String {
    let mut content = String::new();
    input(cur_file).read_to_string(&mut content).unwrap();
    content
}
