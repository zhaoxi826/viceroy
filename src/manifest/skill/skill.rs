use crate::installer::git;
use crate::installer::install::Installer;
use crate::manifest::skill::model::SkillModel;
use std::path::PathBuf;

impl SkillModel{
    fn install(git_repo_url: String, root_cache_path: String, relative_path: String) -> Self{
        let git_installer = git::GitInstaller::new(&git_repo_url, &root_cache_path);
        if let Err(e) = git_installer.download() {
            eprintln!("安装失败: {}", e);
        };
        let git_repo_name = git_repo_url
            .split('/')
            .last()
            .unwrap_or("default_repo")
            .trim_end_matches(".git");
        let mut path_builder = PathBuf::from(&root_cache_path);
        path_builder.push(git_repo_name);
        path_builder.push(&relative_path);
        let skill_path = path_builder.to_string_lossy().to_string();
        Self{
            skill_path,
        }
    }
    fn analysis(&self){
    }
}

