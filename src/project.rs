use crate::args::NewProjectArgs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Project {
    name: String,
    path: PathBuf,
    config_file: Option<PathBuf>,
}

impl Project {}
