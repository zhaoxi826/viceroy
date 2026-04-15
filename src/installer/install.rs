use anyhow::Result;

pub trait Installer {
    fn download(&self) -> Result<()>;
}