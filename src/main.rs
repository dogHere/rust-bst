extern crate bst;

use bst::tree::*;

fn main() {
    let mut tree = Tree::new();
    tree.push("a");
    tree.push("b");
    tree.push("5");
    tree.push("0");
    tree.push("6");
    tree.push("6");
    tree.push("z");

    println!("{:?}", tree);
    println!("{:?}", tree.min());
    println!("{:?}", tree.max());
    println!("{:?}", tree.exists(&"12"));
    println!("{:?}", tree.exists(&"z"));
    println!("{:?}", tree.find(&"123"));
    println!("{:?}", tree.find(&"a"));

    tree.delete("0");
    tree.delete("10");
    tree.delete("b");
    tree.delete("a");


    println!("{:?}", tree);
}
