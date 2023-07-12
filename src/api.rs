// A normal Rust function ...
pub fn draw_tree(root: TreeNode, mode: DrawMode) { /* ... */ }

// ... with rich types
pub struct TreeNode { pub value: String, pub children: Vec<TreeNode> }
pub enum DrawMode { Colorful {palette: String}, Grayscale }