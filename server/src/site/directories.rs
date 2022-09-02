use std::{
    env,
    path::{Path, PathBuf},
};

pub const PUBLIC_FOLDER: &str = "public";

lazy_static! {
    pub static ref CURRENT_DIR: PathBuf =
        env::current_dir().expect("could not get current directory");
    pub static ref PUBLIC_DIR: PathBuf = CURRENT_DIR.join(Path::new(PUBLIC_FOLDER));
}
