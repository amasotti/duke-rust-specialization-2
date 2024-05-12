# Getting started

![Rust project structure](assets/Rust_key_concepts.jpg)


## Useful commands

- All `cargo` commands

## Packages
- [Rayon](https://docs.rs/crate/rayon/latest) - A data parallelism library for Rust

~~~rust
// Basic Rayon parallel map example  
use rayon::prelude::*;

fn main() {
  let vals = vec![1, 2, 3];
  let squared = vals.par_iter() // Rayon parallel iterator 
    .map(|x| x * x)  
    .collect::<Vec<_>>(); 
  
  println!("{:?}", squared);   
}
~~~
