use std::path::Path;

use crate::prelude::*;

// walk starts with the current file or dir and then visits each child file and dir
pub fn walk<F>(start: &str, f: F) -> WalkResult<()>
where
    F: FnMut(String) -> (),
{
    let start = Path::new(start);
    walk_path(start, f)
}

fn walk_path<F>(path: &Path, f: F) -> WalkResult<()>
where
    F: FnMut(String) -> (),
{
    visit_path(path, f)?;
    Ok(())
}

fn visit_path<F>(path: &Path, mut f: F) -> WalkResult<()>
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
