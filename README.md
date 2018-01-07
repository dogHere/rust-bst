# rust-bst

A BST Tree in rust.

# Example

```rust

let  tree = Tree::new()
    .push(String::from("3"))
    .push(String::from("1"))
    .push(String::from("5"))
    .push(String::from("0"))
    .push(String::from("6"))
    .push(String::from("6"));

println!("{:?}",tree);
```