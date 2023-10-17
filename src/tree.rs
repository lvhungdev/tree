use std::fs;
use std::io::{Error, ErrorKind, Result};

use crate::node::{Node, NodeKind};
use crate::path_utils::PathUtils;

pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new(path: &str, max_level: u16) -> Result<Self> {
        if !PathUtils::is_path_directory(path)? {
            return Err(Error::new(ErrorKind::Other, "Path is not a directory"));
        }

        let absolute_path: String = PathUtils::get_absolute_path(path)?;
        let name: String = PathUtils::get_name_from_absolute_path(&absolute_path)?;

        let mut root: Node = Node {
            absolute_path,
            name,
            kind: NodeKind::Directory,
            file_children: Vec::new(),
            dir_children: Vec::new(),
            level: 0,
            has_next_sibling: false,
        };

        Self::build_recursive(&mut root, max_level)?;

        return Ok(Tree { root });
    }

    pub fn print(&self) {
        Self::print_recursive(&self.root, "");
    }

    fn build_recursive(node: &mut Node, max_level: u16) -> Result<()> {
        if node.level >= max_level {
            return Ok(());
        }

        let entries: fs::ReadDir = fs::read_dir(&node.absolute_path)?;

        for entry in entries.into_iter() {
            match entry {
                Ok(entry) => {
                    let absolute_path: String =
                        PathUtils::get_absolute_path(entry.path().to_str().unwrap())?;
                    let name: String = PathUtils::get_name_from_absolute_path(&absolute_path)?;
                    let is_directory: bool = PathUtils::is_path_directory(&absolute_path)?;
                    let kind: NodeKind = if is_directory {
                        NodeKind::Directory
                    } else {
                        NodeKind::File
                    };

                    let mut child: Node = Node {
                        absolute_path,
                        name,
                        kind,
                        dir_children: Vec::new(),
                        file_children: Vec::new(),
                        level: node.level + 1,
                        has_next_sibling: true,
                    };

                    if is_directory {
                        Self::build_recursive(&mut child, max_level)?;
                        node.dir_children.push(child);
                    } else {
                        node.file_children.push(child);
                    }
                }
                Err(_) => {}
            }
        }

        match node.dir_children.last_mut() {
            Some(child) => child.has_next_sibling = node.file_children.len() > 0,
            None => {}
        }
        match node.file_children.last_mut() {
            Some(child) => child.has_next_sibling = false,
            None => {}
        }

        return Ok(());
    }

    fn print_recursive(node: &Node, prefix: &str) {
        print!("{}", prefix);

        if node.has_next_sibling {
            println!("├── {}", node.name);
        } else {
            println!("└── {}", node.name);
        }

        let mut prefix: String = prefix.to_string();
        prefix.push_str(if node.has_next_sibling {
            "│   "
        } else {
            "    "
        });

        for child in &node.dir_children {
            Self::print_recursive(child, &prefix);
        }

        for child in &node.file_children {
            Self::print_recursive(child, &prefix);
        }
    }
}
