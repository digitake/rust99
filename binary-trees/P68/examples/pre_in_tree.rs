use P68::*;

pub fn main() {
    let trees = pre_in_tree(
        &vec!['a', 'b', 'd', 'e', 'c', 'f', 'g'],
        &vec!['d', 'b', 'e', 'a', 'c', 'g', 'f'],
    );
    for tree in trees {
        println!("{}", tree);
    }

    //let trees = pre_in_tree(&vec!['a', 'b', 'a'], &vec!['b', 'a', 'a']);
    //for tree in trees {
    //    println!("{}", tree);
    //}
}
