use std::{env, fs};
use std::fs::{DirEntry, File};
use std::path::Path;
use std::io::Read;

fn ls(path: Option<&Path>) {
    let path = if let Some(path) = path {
        path
    } else {
        &env::current_dir().unwrap()
    };

    let entries = fs::read_dir(path).unwrap();

    let mut entries: Vec<DirEntry> = entries
        .map(|entry| entry.unwrap())
        .collect();

    entries.sort_by(|a, b| a.path().cmp(&b.path()));

    for entry in entries {
        let filename= entry.file_name().into_string().unwrap();

        println!("{}", filename);
    }

}

fn cat(path: &Path) {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        ls(None);
    } else {
        let arg = args.iter().nth(1).unwrap().clone();
        let path = Path::new(&arg);
        if path.is_dir() {
            ls(Some(path));
        } else {
            cat(path);
        }
    }
}
