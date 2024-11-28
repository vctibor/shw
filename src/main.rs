use std::fmt::Debug;
use std::fs;
use std::fs::{DirEntry, File};
use std::path::Path;
use std::io::Read;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show directory entry statistics
    #[arg(short = 'l')]
    stats: bool,

    /// Show hidden files (dot files)
    #[arg(short)]
    all: bool,

    /// Path to directory or file
    #[arg(index = 1, default_value = ".")]
    path: String,
}

fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{}B", size)
    } else if size < 1024 * 1024 {
        format!("{}KB", size / 1024)
    } else if size < 1024 * 1024 * 1024 {
        format!("{}MB", size / 1024 / 1024)
    } else if size < 1024 * 1024 * 1024 * 1024 {
        format!("{}GB", size / 1024 * 1024 * 1024)
    } else {
        format!("{}TB", size / 1024 * 1024 * 1024 * 1024)
    }
}

fn ls(path: &Path, args: &Args) {
    let entries = fs::read_dir(path).unwrap();

    let mut entries: Vec<DirEntry> = entries
        .map(|entry| entry.unwrap())
        .collect();

    entries.sort_by(|a, b| a.path().cmp(&b.path()));

    for entry in entries {
        let filename= entry.file_name().into_string().unwrap();

        if !args.all && filename.starts_with(".") {
            continue;
        }

        if args.stats {
            let metadata = entry.metadata().unwrap();
            let len = format_size(metadata.len());

            //let modified = metadata.modified().unwrap();
            //let permissions = metadata.permissions().

            println!("{} {}", len, filename);
        } else {
            println!("{}", filename);
        }
    }

}

fn cat(path: &Path, args: &Args) {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    if path.is_dir() {
        ls(path, &args);
    } else {
        cat(path, &args);
    }
}
