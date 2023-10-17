pub struct Node {
    pub absolute_path: String,
    pub name: String,
    pub kind: NodeKind,
    pub dir_children: Vec<Node>,
    pub file_children: Vec<Node>,
    pub level: u16,
    pub has_next_sibling: bool,
}

pub enum NodeKind {
    File,
    Directory,
}
