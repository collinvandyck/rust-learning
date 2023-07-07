use std::{
    fs::{self, DirEntry},
    path::Path,
};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Walked<'a> {
    pub name: &'a String,
    pub depth: u32,
    pub last: bool,
    pub start: bool,
    pub lasts: &'a Vec<bool>,
}

// walk starts with the current file or dir and then visits each child file and dir
pub fn walk<F>(args: &Args, mut f: F) -> WalkResult<()>
where
    F: Fn(&Walked),
{
    let start = match &args.dir {
        Some(dir) => dir.to_string(),
        None => ".".to_string(),
    };
    let start = Path::new(&start);
    walk_path(args, start, 0, vec![], &mut f)
}

fn walk_path<'a, F>(
    args: &Args,
    path: &Path,
    depth: u32,
    lasts: Vec<bool>,
    f: &mut F,
) -> WalkResult<()>
where
    F: Fn(&Walked),
{
    let to_str = path.to_string_lossy().to_string();
    if !path.exists() {
        return Err(Error::NotFound(to_str));
    }
    if !path.is_dir() {
        return Err(Error::NotDirectory(to_str));
    }
    if path.is_symlink() {
        // we don't follow symlinks for now
        return Ok(());
    }
    if depth == 0 {
        let walked = Walked {
            name: &path.to_string_lossy().to_string(),
            depth,
            last: true,
            start: true,
            lasts: &vec![true],
        };
        f(&walked);
    }
    if path.is_file() {
        return Ok(());
    }
    let recurse = args.depth.map_or(true, |max_depth| depth < max_depth);
    if recurse && path.is_dir() {
        let read_dir = fs::read_dir(path)?;
        let mut entries = vec![];
        for entry in read_dir {
            let entry = entry?;
            entries.push(entry);
        }
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        let mut iter = entries.iter().filter(|s| filter(args, s)).peekable();
        while let Some(entry) = iter.next() {
            let last = iter.peek().is_none();
            let path = entry.path();
            let name = path_to_file_name(&path)?;
            let walked = Walked {
                name: &name,
                lasts: &lasts,
                start: false,
                depth,
                last,
            };
            f(&walked);
            if path.is_dir() {
                let mut lasts = lasts.clone();
                lasts.push(last);
                walk_path(args, &path, depth + 1, lasts, f)?;
            }
        }
    }
    Ok(())
}

fn filter(args: &Args, entry: &DirEntry) -> bool {
    match entry.file_name().to_str() {
        Some(s) => {
            if !args.show_hidden && s.starts_with('.') {
                false
            } else {
                true
            }
        }
        None => false,
    }
}

fn path_to_file_name(p: &Path) -> WalkResult<String> {
    p.file_name()
        .ok_or(Error::NoFileName)
        .and_then(|f| Ok(f.to_string_lossy().to_string()))
}
