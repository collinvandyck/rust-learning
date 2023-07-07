use std::{fs, path::Path};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Walked {
    pub name: String,
    pub depth: u32,
    pub last: bool,
    pub first: bool,
    pub root_last: bool,
}

// walk starts with the current file or dir and then visits each child file and dir
pub fn walk<F>(start: &str, max_depth: Option<u32>, mut f: F) -> WalkResult<()>
where
    F: FnMut(Walked),
{
    let start = Path::new(start);
    walk_path(start, 0, max_depth, false, &mut f)
}

fn walk_path<'a, F>(
    path: &Path,
    depth: u32,
    max_depth: Option<u32>,
    mut root_last: bool,
    f: &mut F,
) -> WalkResult<()>
where
    F: FnMut(Walked),
{
    if !path.exists() {
        let to_str = path.to_string_lossy().to_string();
        return Err(Error::NotFound(to_str));
    }
    if path.is_symlink() {
        // we don't follow symlinks for now
        return Ok(());
    }
    if path.is_file() {
        // if we are here, that means that the only walked result is a file.
        let name = path_to_file_name(path)?;
        let walked = Walked {
            name,
            depth,
            root_last,
            last: true,
            first: true,
        };
        f(walked);
        return Ok(());
    }
    let recurse = max_depth.map_or(true, |max_depth| depth < max_depth);
    if recurse && path.is_dir() {
        let entries = fs::read_dir(path)?;
        let mut iter = entries.enumerate().peekable();
        while let Some((idx, entry)) = iter.next() {
            let entry = entry?;
            let first = idx == 0;
            let last = iter.peek().is_none();
            let path = entry.path();
            let name = path_to_file_name(&path)?;
            let walked = Walked {
                name,
                depth,
                last,
                first,
                root_last,
            };
            f(walked);
            if path.is_dir() {
                if !root_last && depth == 0 && last {
                    root_last = true
                }
                walk_path(&path, depth + 1, max_depth, root_last, f)?;
            }
        }
    }
    Ok(())
}

fn path_to_file_name(p: &Path) -> WalkResult<String> {
    p.file_name()
        .ok_or(Error::NoFileName)
        .and_then(|f| Ok(f.to_string_lossy().to_string()))
}
