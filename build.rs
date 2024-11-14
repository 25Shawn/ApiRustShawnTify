use std::fs;
use std::path::Path;

fn main() {
    let src_dir = Path::new("src/musiques");
    let dest_dir = Path::new("target/debug/deps/musiques");

    if let Err(e) = fs::create_dir_all(dest_dir) {
        panic!("Failed to create directory: {}", e);
    }

    for entry in src_dir.read_dir().expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let src_path = entry.path();
        let file_name = src_path.file_name().expect("Failed to get file name");
        let dest_path = dest_dir.join(file_name);

        if let Err(e) = fs::copy(&src_path, &dest_path) {
            panic!("Failed to copy file: {}", e);
        }
    }
}
