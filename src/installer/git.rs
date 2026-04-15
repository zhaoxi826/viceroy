/*
 * // Copyright 2026 zhaoxi826
 * //
 * // Licensed under the Apache License, Version 2.0 (the "License");
 * // you may not use this file except in compliance with the License.
 * // You may obtain a copy of the License at
 * //
 * //     http://www.apache.org/licenses/LICENSE-2.0
 * //
 * // Unless required by applicable law or agreed to in writing, software
 * // distributed under the License is distributed on an "AS IS" BASIS,
 * // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * // See the License for the specific language governing permissions and
 * // limitations under the License.
 */

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
    pub fn new(git_repo_url: &str, root_cache_path: &str) -> Self {
        let repo_name = git_repo_url
            .split('/')
            .last()
            .unwrap_or("default_repo")
            .trim_end_matches(".git");
        let mut path_buf = PathBuf::from(root_cache_path);
        path_buf.push(repo_name);
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
