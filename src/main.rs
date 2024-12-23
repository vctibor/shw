use std::ffi::OsString;
use std::fmt::Debug;
use std::fs;
use std::fs::{DirEntry, File};
use std::path::Path;
use std::io::Read;
use derive_more::From;
use clap::Parser;
use owo_colors::OwoColorize;

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

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
enum Error {
    #[from]
    Other {
        message: String
    },

    #[from]
    Io(std::io::Error),

    #[from]
    OsString {
        message: OsString
    },
}

fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{} KiB", size / 1024)
    } else if size < 1024 * 1024 * 1024 {
        format!("{} MiB", size / 1024 / 1024)
    } else if size < 1024 * 1024 * 1024 * 1024 {
        format!("{} GiB", size / 1024 * 1024 * 1024)
    } else {
        format!("{} TiB", size / 1024 * 1024 * 1024 * 1024)
    }
}

fn print_entry(entry: &DirEntry, args: &Args, dir: bool) -> Result<()> {
    let filename= entry.file_name().into_string()?;

    if !args.all && filename.starts_with(".") {
        return Ok(());
    }

    let output = if args.stats {
        let metadata = entry.metadata()?;
        let len = format_size(metadata.len());

        let _modified = metadata.modified()?;
        let _permissions = metadata.permissions();

        format!("{} {}", len, filename)
    } else {
        filename
    };

    if dir {
        println!("{}", output.blue());
    } else {
        println!("{}", output);
    }

    Ok(())
}

// TODO: Directory size by subentries, not inode size
fn ls(path: &Path, args: &Args) -> Result<()> {
    let mut directories = Vec::new();
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            directories.push(entry);
        } else {
            files.push(entry);
        }
    }

    directories.sort_by(|a, b| a.path().cmp(&b.path()));
    files.sort_by(|a, b| a.path().cmp(&b.path()));

    for dir in directories {
        let _ = print_entry(&dir, args, true);
    }

    for file in files {
        let _ = print_entry(&file, args, false);
    }

    Ok(())
}

// TODO: Syntax coloring
fn cat(path: &Path, _args: &Args) -> Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", contents);

    Ok(())
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);

    let res = if path.is_dir() {
        ls(path, &args)
    } else {
        cat(path, &args)
    };

    if let Err(e) = res {
        println!("Error: {:?}", e);
    }
}
