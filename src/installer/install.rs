use anyhow::Result;
use std::path::Path;
use std::fs;

pub trait Installer {
    fn download(&self) -> Result<()>;
}

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target = dst.join(entry.file_name());

        // Skip .git directory to avoid unnecessary weight
        if entry.file_name() == ".git" {
            continue;
        }

        if ty.is_dir() {
            copy_dir_recursive(&entry.path(), &target)?;
        } else {
            fs::copy(&entry.path(), &target)?;
        }
    }
    Ok(())
}