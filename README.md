# rust-bst

A BST Tree in rust(Keep writing).

# Example

```rust
extern crate bst;

use bst::tree::*;

fn main() {
    let mut tree = Tree::new();
    tree.push("a")
        .push("b")
        .push("5")
        .push("0")
        .push("6")
        .push("6")
        .push("z");

    println!("{:?}", tree);
    println!("{:?}", tree.min());
    println!("{:?}", tree.max());
    println!("{:?}", tree.exists(&"12"));
    println!("{:?}", tree.exists(&"z"));
    println!("{:?}", tree.find(&"123"));
    println!("{:?}", tree.find(&"a"));

    tree.delete(&"0")
        .delete(&"10")
        .delete(&"b")
        .delete(&"a");

    println!("{:?}", tree);
}


```

# License
   Copyright 2017 doghere

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
