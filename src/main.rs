use md5;
use std::env;
use std::fs::{self};
use std::io::prelude::*;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::time::Instant;

fn main() -> Result<(), io::Error> {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let mut scan_dir = String::from(".");

    if args.len() >= 2 {
        scan_dir = args[1].clone();
    }

    let initial_dir = Path::new(&scan_dir);

    if !initial_dir.is_dir() {
        panic!("Invalid directory {}", initial_dir.display());
    }

    match walk_dir(&initial_dir) {
        Ok(files) => {
            let  stdout = io::stdout();
            let mut handle = stdout.lock();

            for file in &files {
                if file.is_dir() {
                    continue;
                }

                let bytes = fs::read(file)?;
                let file_length = bytes.len();
                let hash = md5::compute(&bytes);
                if let Err(e) = writeln!(handle, "{}\t{:?}\t{}", file.display(), hash, file_length)
                {
                    eprintln!("{}", e);
                }
            }

            drop(handle);
            eprintln!("Hashed {} files in {}ms!", files.len(), start.elapsed().as_millis());
        }
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}

fn walk_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_files = walk_dir(&path)?;
            files.extend(sub_files);
        } else {
            if path.display().to_string().contains("\t") {
                panic!("File {} contains a forbidden tab character.", path.display());
            }

            files.push(path.clone());
        }
    }

    Ok(files)
}
