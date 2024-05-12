# Course 2 - Data Engineering with Rust

Part of the Coursera
specialization [Rust Programming Specialization](https://www.coursera.org/specializations/rust-programming) (Duke
University).
This is the second course of the specialization, which focuses on data engineering with Rust. It is all about performant
handling of date, ETL pipelines and MLOps.

__Instructor__: [Noah Gift](https://noahgift.com/)

### Structure

- [Week1](week1) - Introduction

## Run

I've chosen to organize my snippets in separate cargo projects, under the directories
named after the course week. They don't follow exactly the same structure as the course.

The [just](https://just.systems/man/en/chapter_1.html) file is used to confortably run the snippets
in the different subfolders without having to manually navigate to them.

```sh
# In the justfile I've given custom names to the subfolders (see below)
# simple-api -> week1/0-simple-rust-api/simple-api

# To run them, just type
just run simple-api 

# To test them
just test simple-api

# To build
just build simple-api

# To lint
just lint simple-api 

# To clean
just clean simple-api
```

Some aliases are defined (just lazyness):

- `just t` -> `just test`
- `just r` -> `just run`
- `just b` -> `just build`
- `just l` -> `just lint`
- `just c` -> `just clean`

## Resources

- [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

### Specific topics


### Coursera Discussions

- [Week1 - Disappointment](https://www.coursera.org/learn/data-engineering-rust/discussions/forums/kUkT2yC8Ee6_Lgoc7ulpMw/threads/RROzRfxWEe66og4Z2poFYQ)
