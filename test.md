# Course 1: Rust Fundamentals

Part of the Coursera specialization [Rust Programming Specialization](https://www.coursera.org/specializations/rust-programming) (Duke University).

## Run

I've chosen to organize my snippets in separate cargo projects, under the directories
named after the course week. They don't follow exactly the same structure as the course.

The [just](https://just.systems/man/en/chapter_1.html) file is used to confortably run the snippets
in the different subfolders without having to manually navigate to them.

```sh
# In the justfile I've given custom names to the subfolders (see below)
# basics -> week2/1-basics/proj 
# control-flow -> week2/2-control-flow/proj
# functions -> week2/3-functions/proj

# To run them, just type
just run basics # or control-flow or functions
# identical to `cd week2/1-basics/proj && cargo run`

# To test them
just test basics

# To build
just build basics

# To lint
just lint basics 

# To clean
just clean basics
```

Some aliases are defined (just lazyness):

- `just t` -> `just test`
- `just r` -> `just run`
- `just b` -> `just build`

### Structure

- Week 1: Was mostly about rust toolchain setup, devcontainers, codespaces etc... I knew them before, so I 
  had a quick look and moved on.
- [Week2](./week2):
  - [Basics of Rust](./week2/1-basics/proj): Variables, types
  - [Control Flow](./week2/2-control-flow/proj): If, loops, match
  - [Functions](./week2/3-functions/proj): Functions, methods, error handling, panic

- [Week3](./week3):
  - [Structs](./week3/1-structured-data/proj): Structs
    - [Struct Lab](./week3/2-struct-lab/proj): Structs lab
    - [Associated Functions](./week3/3-methods-lab/proj): Associated functions
  - [Strings](./week3/4-strings/proj): Strings
    - [Strings Lab](./week3/5-strings-lab/proj): Strings lab
  - [Vectors](week3/6-intro-to-vectors/proj): Vectors
    - [Vectors Lab](week3/7-lab-vectors/proj): Vectors lab
  - [Enums](week3/8-enums/proj): Enums
    - [Enums Lab](week3/9-lab-enums/proj): Enums lab

- [Week4](./week4):
  - [Working with libraries](./week4/1-create-lib/proj): Crates, modules

## Resources

- [The Book](https://doc.rust-lang.org/book/)
- [Microsoft Learn Rust](https://learn.microsoft.com/en-us/training/modules/rust-introduction/)
- [Microsoft Rust First Steps](https://learn.microsoft.com/en-us/training/paths/rust-first-steps/)

### Specific topics

- [Loops and Hashmaps](https://learn.microsoft.com/en-us/training/modules/rust-loop-expressions/)
- [Manage Modules & Crates (RustDocs)](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
