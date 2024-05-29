use std::collections::HashMap;
// Define the structure for a File
#[derive(Debug)]
struct File {
    name: String, //TODO: make sha384 digest :ODOT//
    path: String,
}

// Define the structure for a Directory
#[derive(Debug)]
struct Directory {
    parent:Box<Node>,
    files: Vec<File>,
    subdirectories: HashMap<String, Node>,
}

// Define a node of the B-tree
#[derive(Debug)]
enum Node {
    File(File),
    Directory(Box<Directory>),
}

// Define the B-tree structure
#[derive(Debug)]
struct BTree {
    root: Node,
}

// Implement methods for BTree
impl BTree {
    fn new(root: Node) -> Self {
        BTree { root }
    }

    fn print(&self) {
        self.print_recursive(&self.root, 0);
    }

    fn print_recursive(&self, node: &Node, depth: usize) {
        match node {
            Node::File(file) => {
                println!("{}{}", "-".repeat(depth), file.name);
            }
            Node::Directory(dir) => {
                println!("{}{:#?}", "-".repeat(depth), dir.parent);
                for file in &dir.files {
                    println!("{}{}", "-".repeat(depth + 1), file.name);
                }
                for (_, subdir) in &dir.subdirectories {
                    self.print_recursive(subdir, depth + 1);
                }
            }
        }
    }
}
