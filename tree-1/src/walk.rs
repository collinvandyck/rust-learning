use std::{fs, path::Path};

use crate::prelude::*;

// walk starts with the current file or dir and then visits each child file and dir
pub fn walk<F>(start: &str, max_depth: &Option<u32>, mut f: F) -> WalkResult<()>
where
    F: FnMut(String) -> (),
{
    let start = Path::new(start);
    walk_path(start, 1, max_depth, &mut f)
}

fn walk_path<F>(path: &Path, depth: u32, max_depth: &Option<u32>, f: &mut F) -> WalkResult<()>
where
    F: FnMut(String) -> (),
{
    visit_path(path, f)?;
    let recurse = match max_depth {
        Some(max_depth) => depth < *max_depth,
        None => true,
    };
    if recurse && path.is_dir() {
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            walk_path(&path, depth + 1, max_depth, f)?;
        }
    }
    Ok(())
}

fn visit_path<F>(path: &Path, f: &mut F) -> WalkResult<()>
where
    F: FnMut(String) -> (),
{
    let pstr = check_path(path)?;
    f(pstr);
    Ok(())
}

fn check_path(path: &Path) -> WalkResult<String> {
    let pstr = to_string(path);
    if !path.exists() {
        return Err(Error::NotFound(pstr.clone()));
    }
    Ok(pstr)
}

fn to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}
