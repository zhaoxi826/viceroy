use crate::installer::install::Installer;
use std::process::Command;
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct GitInstaller{
    git_repo_url: String,
    cache_path: Option<String>,
}

impl GitInstaller {
    // 1. 增加一个关联函数 new，负责“一键初始化”
    pub fn new(git_repo_url: &str, root_cache_path: &str) -> Self {
        // 逻辑：从 URL 提取仓库名（总督的直觉）
        let repo_name = git_repo_url
            .split('/')
            .last()
            .unwrap_or("default_repo")
            .trim_end_matches(".git");
        let mut path_buf = PathBuf::from(root_cache_path);
        path_buf.push(repo_name);

        // 返回实例
        Self {
            git_repo_url: git_repo_url.to_owned(),
            cache_path: Some(path_buf.display().to_string()),
        }
    }
}
impl Installer for GitInstaller{
    fn download(&self) -> Result<()> {
        let cache_path = self.cache_path.as_ref()
            .context("错误：未初始化缓存路径，请先调用 get_cache_path")?;
        if Path::new(cache_path).exists() {
            fs::remove_dir_all(cache_path)
                .with_context(|| format!("无法清理旧的缓存目录: {}", cache_path))?;
        };
        println!("viceroy正在安装");
        let output = Command::new("git")
            .args(["clone",
                "--depth",
                "1",
                &self.git_repo_url,
                cache_path])
            .output()
            .context("执行 Git 失败，请确认系统已安装 git 并配置了 SSH/HTTP 权限")?;
        if output.status.success() {
            println!("✅ 技能包拉取成功。");
            Ok(())
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Git 克隆失败: {}", err_msg))
        }
    }
}
