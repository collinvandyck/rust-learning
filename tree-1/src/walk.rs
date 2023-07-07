use std::env;

use crate::prelude::*;

// walk starts with the current file or dir and then visits each child file and dir
pub fn walk(_dir: &str) -> WalkResult<()> {
    let f = env::current_dir();
    Ok(())
}
