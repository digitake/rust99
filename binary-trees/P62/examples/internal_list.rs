use bintree::Tree;
use P62::*;

pub fn main() {
    let tree = Tree::node(
        'a',
        Tree::leaf('b'),
        Tree::node('c', Tree::leaf('d'), Tree::leaf('e')),
    );
    println!("{:?}", internal_list(&tree));
}
