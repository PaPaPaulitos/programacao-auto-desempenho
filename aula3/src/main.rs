use std::fs;
use std::thread;

mod hash;
use hash::Hash;

fn list_files_using_rust(dir: &str) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    paths
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                path.file_name()?.to_str().map(String::from)
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let files = list_files_using_rust("C:/temp");
    for file_name in files {
        let path = format!("C:/temp/{}", file_name);
        thread::spawn(move || {
            let hash = Hash::new(&path);
            hash.executar().unwrap_or_else(|err| eprintln!("{:?}", err));
        });
    }
}
