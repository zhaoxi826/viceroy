use crate::manifest::skill::skill_structure_tree::{SkillNode, SkillJson, PythonFuncNode, PythonFileNode};
use walkdir::WalkDir;
use std::path::Path;
use std::collections::HashMap;
use ruff_python_parser::{parse, Mode};
use ruff_python_ast::{Mod, Stmt};
use regex::Regex;

pub fn parse_skill_md(content: &str) -> SkillJson {
    let mut metadata = SkillJson::default();

    // Pattern to match YAML frontmatter between `---` and `---`
    let re_frontmatter = Regex::new(r"(?s)^---\s*(.*?)\s*---").unwrap();
    if let Some(caps) = re_frontmatter.captures(content) {
        let frontmatter = caps.get(1).map_or("", |m| m.as_str());
        for line in frontmatter.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let k = key.trim();
                let v = value.trim();
                if k == "name" {
                    metadata.name = v.to_string();
                } else if k == "description" {
                    metadata.description = v.to_string();
                }
            }
        }

        // Extract instructions (everything after the frontmatter)
        let body = re_frontmatter.replace(content, "").trim().to_string();
        metadata.instructions = body;
    } else {
        // No frontmatter found, whole file is instructions
        metadata.instructions = content.trim().to_string();
    }

    metadata
}

fn analyze_python_file(code: &str) -> PythonFileNode {
    let mut func_dict = HashMap::new();
    let parsed: Mod = parse(code, Mode::Module).expect("Python 语法错误");
    if let Mod::Module(module) = parsed {
        for stmt in module.body {
            if let Stmt::FunctionDef(func) = stmt {
                let func_name = func.name.to_string();
                let args = HashMap::new();
                let doc = "从 AST 里抠出来的文档".to_string();
                func_dict.insert(func_name.clone(), PythonFuncNode {
                    func_sign: format!("def {}(...)", func_name),
                    func_args: args,
                    func_return_type: "Unknown".into(),
                    func_docs: doc,
                });
            }
        }
    }
    PythonFileNode {
        file_name: "xxx".into(),
        func_dict,
    }
}

pub fn analyze_skill_directory(root_path: &Path) -> (Option<SkillJson>, SkillNode) {
    let mut root_node = SkillNode::new_folder("root");
    let mut skill_metadata = None;

    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name_str = path.file_name().unwrap().to_string_lossy().to_string();

            // Skip useless files
            if file_name_str.to_lowercase() == "license" || file_name_str.starts_with('.') {
                continue;
            }

            let rel_path = path.strip_prefix(root_path).unwrap();
            let segments: Vec<&str> = rel_path.iter().map(|s| s.to_str().unwrap()).collect();

            if file_name_str.to_lowercase() == "skill.md" && segments.len() == 1 {
                let code = std::fs::read_to_string(path).expect("读取文件失败");
                skill_metadata = Some(parse_skill_md(&code));
                // We don't add SKILL.md to the tree since it will be in skill.json
                continue;
            }

            let node = if path.extension().and_then(|s| s.to_str()) == Some("py") {
                let code = std::fs::read_to_string(path).unwrap_or_default();
                let mut py_node = analyze_python_file(&code);
                py_node.file_name = file_name_str.clone();
                SkillNode::Python(py_node)
            } else {
                SkillNode::File(file_name_str)
            };
            root_node.insert_recursive(&segments, node);
        }
    }

    (skill_metadata, root_node)
}

pub fn process_and_save_skill(root_path: &Path) -> anyhow::Result<()> {
    let (metadata_opt, tree) = analyze_skill_directory(root_path);

    // Save skill.json
    if let Some(metadata) = metadata_opt {
        let skill_json_path = root_path.join("skill.json");
        let skill_json_content = serde_json::to_string_pretty(&metadata)?;
        std::fs::write(&skill_json_path, skill_json_content)?;
        println!("Saved {:?}", skill_json_path);
    }

    // Save metadata.json
    let metadata_json_path = root_path.join("metadata.json");
    let tree_json_content = serde_json::to_string_pretty(&tree)?;
    std::fs::write(&metadata_json_path, tree_json_content)?;
    println!("Saved {:?}", metadata_json_path);

    Ok(())
}
