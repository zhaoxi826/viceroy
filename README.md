
# Viceroy (总督)

Pretor的插件管理工具

![Rust](https://img.shields.io/badge/language-rust-orange.svg)
![Version](https://img.shields.io/badge/version-v0.1-blue.svg)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

---
>*"你们搞大模型的就是码奸，你们已经害死前端兄弟了，还要害死后端兄弟，测试兄弟，运维兄弟，害死网安兄弟，害死ic兄弟，最后害死自己害死全人类"*

viceroy 是一个由rust编写的安装工具，用于pretor的插件管理  
pretor项目仓库：https://github.com/zhaoxi826/Pretor

---
## 目前支持对象
- skill: 安装skill并进行简单的解析到目标文件夹下

---
## 使用方法
#### Skill
**Skill** 是一个由指令、脚本和资源组成的集合，Agent通过动态加载这些内容，以在特定任务上提升表现。**Skill** 教会 **Agent** 如何以可重复的方式完成特定任务，例如按照公司品牌指南创建文档、使用组织特定的工作流程分析数据，或自动化个人任务。  
目标仓库：https://github.com/anthropics/skills
```Bash
./viceroy install (github仓库名) [-p (仓库内SKILL.md所在目录的相对路径)] -o (输出路径)
```
**viceroy**将在skill根目录下产生 **skill.json** 和 **metadata.json**两个文件。  
**skill.json**包括SKILL.md的**name**,**description**,**instructions**。  
**metadata**包含整个skill的文件树和架构和python脚本工具的函数信息。