use rocket::request::FromParam;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

use rand::{self, Rng};

#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

impl PasteId<'_> {
    pub fn new(size: usize) -> PasteId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        PasteId(Cow::Owned(id))
    }

    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        Path::new(root).join(self.0.as_ref())
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = String;
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if param.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(PasteId(param.into()))
        } else {
            Err(format!("ERROR: {}", param.to_string()))
        }
    }
}
