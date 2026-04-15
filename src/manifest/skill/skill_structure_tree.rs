use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    pub instructions: String,
}

#[derive(Serialize)]
pub struct PythonFuncNode{
    pub func_sign: String,
    pub func_args: HashMap<String, String>,
    pub func_return_type: String,
    pub func_docs: String,
}

#[derive(Serialize)]
pub struct PythonFileNode{
    pub file_name: String,
    pub func_dict: HashMap<String, PythonFuncNode>,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
pub enum SkillNode {
    Folder(String, HashMap<String, SkillNode>),
    Python(PythonFileNode),
    File(String),
}

impl SkillNode {
    pub fn new_folder(name: &str) -> Self {
        SkillNode::Folder(name.to_string(), HashMap::new())
    }

    pub fn insert_recursive(&mut self, segments: &[&str], data: SkillNode) {
        if let SkillNode::Folder(_name, children) = self {
            let current_seg = segments[0];
            if segments.len() == 1 {
                children.insert(current_seg.to_string(), data);
            } else {
                let next_node = children
                    .entry(current_seg.to_string())
                    .or_insert_with(|| SkillNode::new_folder(current_seg));

                next_node.insert_recursive(&segments[1..], data);
            }
        } else {
            panic!("试图在已存在文件{}内保存文件", segments[0]);
        }
    }
}