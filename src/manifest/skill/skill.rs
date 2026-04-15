use crate::installer::git;
use crate::installer::install::Installer;
use crate::manifest::skill::model::SkillModel;
use std::path::PathBuf;

impl SkillModel{
    pub fn install(git_repo_url: String, root_cache_path: String, relative_path: String, output_dir: Option<String>) -> Self{
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

        let mut final_path = path_builder.to_string_lossy().to_string();

        if let Some(out_dir) = output_dir {
            // Copy the contents to the output directory
            if let Err(e) = crate::installer::install::copy_dir_recursive(&path_builder, std::path::Path::new(&out_dir)) {
                eprintln!("复制到目标文件夹失败: {}", e);
            } else {
                final_path = out_dir;
            }
        }

        Self{
            skill_path: final_path,
        }
    }
    pub fn analysis(&self) -> anyhow::Result<()> {
        use std::path::Path;
        use crate::manifest::skill::analysis::process_and_save_skill;
        process_and_save_skill(Path::new(&self.skill_path))
    }
}

