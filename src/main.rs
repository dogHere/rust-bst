extern crate bst;

use bst::tree::*;

fn main() {

    let  tree = Tree::new()
        .push("a")
        .push("b")
        .push("5")
        .push("0")
        .push("6")
        .push("6")
        .push("z");

    println!("{:?}",tree);
    println!("{:?}",tree.min());
    println!("{:?}",tree.max());
    println!("{:?}",tree.find("123"));
    println!("{:?}",tree.find("a"));

    let tree = tree
        .delete("5")
        .delete("10")
        .delete("b");

    println!("{:?}",tree);

}
