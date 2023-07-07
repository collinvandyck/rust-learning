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

fn walk_path<F>(path: &Path, mut f: F) -> WalkResult<()>
where
    F: FnMut(String) -> (),
{
    let s = path.to_string_lossy().to_string();
    f(s);
    Ok(())
}
