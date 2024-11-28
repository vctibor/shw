use std::{env, fs};
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn ls(path: Option<&Path>) {
    let path = if let Some(path) = path {
        path
    } else {
        &env::current_dir().unwrap()
    };

    let paths = fs::read_dir(path).unwrap();
    paths.for_each(|entry| {
        let path_display = entry.unwrap().path();
        println!("{}", path_display.display());
    })
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
