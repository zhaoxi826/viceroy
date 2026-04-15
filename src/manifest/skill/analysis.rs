use crate::manifest::skill::skill_structure_tree::{SkillNode,PythonFuncNode,PythonFileNode};
use walkdir::WalkDir;
use std::path::Path;
use std::collections::HashMap;
use ruff_python_parser::{parse, Mode};
use ruff_python_ast::{Mod, Stmt};

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

fn main_scan_logic(root_path: &Path) {
    let mut root_node = SkillNode::new_folder("root");
    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let rel_path = path.strip_prefix(root_path).unwrap();
            let segments: Vec<&str> = rel_path.iter().map(|s| s.to_str().unwrap()).collect();
            let node = if path.extension().and_then(|s| s.to_str()) == Some("py") {
                let code = std::fs::read_to_string(path).expect("读取文件失败");
                let mut py_node = analyze_python_file(&code);
                py_node.file_name = path.file_stem().unwrap().to_string_lossy().into();
                SkillNode::Python(py_node)
            } else {
                let file_name = path.file_name().unwrap().to_string_lossy().into();
                SkillNode::File(file_name)
            };
            root_node.insert_recursive(&segments, node);
        }
    }
    let json = serde_json::to_string_pretty(&root_node).unwrap();
    println!("{}", json);
}
