use helpers;

pub fn main() -> u32 {
    let tree = helpers::get_tree("./src/problem0067/triangle.txt");

    helpers::max_tree_path(&tree)
}
