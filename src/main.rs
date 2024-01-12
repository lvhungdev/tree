mod node;
mod path_utils;
mod tree;

use std::env;
use tree::Tree;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut path: String = ".".to_string();
    let mut level: u16 = 3;

    for i in (0..args.len()).step_by(2) {
        match args[i].as_str() {
            "--path" | "-p" => match args.get(i + 1) {
                Some(p) => path = p.to_string(),
                None => println!("[WARNING] No path specified"),
            },
            "--level" | "-l" => match args.get(i + 1) {
                Some(l) => match l.parse::<u16>() {
                    Ok(l) => level = l,
                    Err(_) => println!("[WARNING] Invalid level"),
                },
                None => println!("[WARNING] No level specified"),
            },
            other => println!("[WARNING] Unknown argument: {}", other),
        }
    }

    println!();

    let tree: Tree = Tree::build(&path, level).expect("Something wrong");

    tree.print();
}
