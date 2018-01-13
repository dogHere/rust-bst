//! A simple bst tree.
//! Just for fine.
//! # Example
//! ```
//! extern crate bst_rs;
//!
//! use bst_rs::tree::*;
//!
//! fn main() {
//!     let mut tree = Tree::new();
//!     tree.push("a")
//!         .push("b")
//!         .push("5")
//!         .push("0")
//!         .push("6")
//!         .push("6")
//!         .push("z");
//!
//!     println!("{:?}", tree);
//!     println!("{:?}", tree.min());
//!     println!("{:?}", tree.max());
//!     println!("{:?}", tree.exists(&"12"));
//!     println!("{:?}", tree.exists(&"z"));
//!     println!("{:?}", tree.find(&"123"));
//!     println!("{:?}", tree.find(&"a"));
//!
//!     tree.delete(&"0")
//!         .delete(&"10")
//!         .delete(&"b")
//!         .delete(&"a");
//!
//!     println!("{:?}", tree);
//! }
//! ```
pub mod tree;