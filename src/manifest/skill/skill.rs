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
            // Determine the name of the skill directory to create inside the output directory.
            // e.g. if relative_path is "skills/skill-creator", skill_dir_name is "skill-creator".
            // If relative_path is empty, use the repo name.
            let skill_dir_name = if relative_path.is_empty() {
                git_repo_name
            } else {
                relative_path.split('/').last().unwrap_or(git_repo_name)
            };

            let target_dst = std::path::Path::new(&out_dir).join(skill_dir_name);

            // Copy the contents to the new target directory
            if let Err(e) = crate::installer::install::copy_dir_recursive(&path_builder, &target_dst) {
                eprintln!("复制到目标文件夹失败: {}", e);
            } else {
                final_path = target_dst.to_string_lossy().to_string();
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

